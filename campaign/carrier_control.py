#!/usr/bin/env python3
"""In-process, run-bound QMP set_link controller for two explicit guest cases."""
import json,os,re,socket,stat,time
from pathlib import Path

class CarrierController:
    def __init__(self,path,serial,run_id,case_id,log):
        self.path=Path(path);self.serial=Path(serial);self.run_id=run_id
        self.limit={'G09':2,'G15':6}[case_id];self.sequence=0;self.position=0;self.tail=''
        self.channel=None;self.reader=None;self.socket_inode=None
        self.log=Path(log).open('x')
        self.record('created',run_id=run_id,case_id=case_id,socket=str(path),max_events=self.limit)
    def record(self,kind,**values):
        self.log.write(json.dumps(dict(kind=kind,monotonic=time.monotonic(),**values))+'\n')
        self.log.flush();os.fsync(self.log.fileno())
    def connect(self):
        deadline=time.monotonic()+5
        while True:
            try:
                channel=socket.socket(socket.AF_UNIX,socket.SOCK_STREAM);channel.settimeout(2)
                channel.connect(str(self.path));break
            except (FileNotFoundError,ConnectionRefusedError):
                channel.close()
                if time.monotonic()>deadline:raise RuntimeError('QMP socket unavailable')
                time.sleep(.05)
        self.channel=channel;self.reader=channel.makefile('rb')
        info=self.path.lstat()
        if not stat.S_ISSOCK(info.st_mode) or info.st_uid!=os.getuid() or info.st_mode&0o077:
            raise RuntimeError('QMP socket ownership/mode mismatch')
        self.socket_inode=info.st_ino
        greeting=self.receive();self.record('greeting',response=greeting)
        if 'QMP' not in greeting:raise RuntimeError('invalid QMP greeting')
        self.command('qmp_capabilities',{},'capabilities')
    def receive(self):
        data=self.reader.readline(65537)
        if not data or len(data)>65536:raise RuntimeError('missing/oversized QMP record')
        return json.loads(data)
    def command(self,name,arguments,identity):
        request={'execute':name,'arguments':arguments,'id':identity}
        self.record('request',request=request)
        self.channel.sendall((json.dumps(request)+'\n').encode())
        for _ in range(32):
            response=self.receive();self.record('response',response=response)
            if response.get('id')==identity:
                if 'error' in response or 'return' not in response:raise RuntimeError('QMP command refused')
                return response
        raise RuntimeError('bounded QMP response limit')
    def accept(self,line):
        if not line.startswith('ALIBI_CARRIER_REQUEST '):return
        match=re.fullmatch(r'ALIBI_CARRIER_REQUEST ([A-Za-z0-9_-]+) ([0-9]+) (down|up)',line)
        if not match or match[1]!=self.run_id:raise RuntimeError('carrier request identity mismatch')
        sequence=int(match[2]);state=match[3]
        if sequence!=self.sequence+1 or sequence>self.limit or state!=('down' if sequence%2 else 'up'):
            raise RuntimeError('carrier request ordering/boundary mismatch')
        self.record('guest_request',line=line)
        self.command('set_link',{'name':'labnic0','up':state=='up'},self.run_id+'-'+str(sequence))
        self.sequence=sequence
    def poll(self):
        if not self.serial.exists():return
        with self.serial.open('r',errors='replace') as f:
            f.seek(self.position);chunk=f.read(65536);self.position=f.tell()
        self.tail+=chunk
        while '\n' in self.tail:
            line,self.tail=self.tail.split('\n',1);self.accept(line.rstrip('\r'))
        if len(self.tail)>65536:raise RuntimeError('unbounded serial request line')
    def close(self):
        if self.reader:self.reader.close()
        if self.channel:self.channel.close()
        disposition='absent-after-QEMU-exit'
        if self.path.exists():
            info=self.path.lstat()
            if not stat.S_ISSOCK(info.st_mode) or info.st_uid!=os.getuid() or (self.socket_inode is not None and info.st_ino!=self.socket_inode):
                raise RuntimeError('QMP socket identity changed; retained for review')
            self.path.unlink();disposition='exact-owned-socket-removed'
        self.path.parent.rmdir()
        self.record('retired',socket_disposition=disposition,events=self.sequence)
        self.log.close()
        return {'events':self.sequence,'expected_events':self.limit,'socket_disposition':disposition,'directory_removed':True}
