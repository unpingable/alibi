#define _POSIX_C_SOURCE 200809L
/* Compile on the development host; execute ONLY inside the admitted guest. */
#include <errno.h>
#include <fcntl.h>
#include <linux/if.h>
#include <linux/if_tun.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <time.h>
#include <unistd.h>

static double uptime(void) {
    struct timespec t;
    if (clock_gettime(CLOCK_MONOTONIC, &t) != 0) exit(70);
    return (double)t.tv_sec + (double)t.tv_nsec / 1000000000.0;
}
static int phase(const char *name) {
    FILE *f = fopen("/run/alibi-tun/phase.tmp", "w");
    if (!f) return -1;
    fprintf(f, "%s\n", name);
    if (fclose(f) != 0 || rename("/run/alibi-tun/phase.tmp", "/run/alibi-tun/phase") != 0) return -1;
    printf("TUN_PHASE %s uptime=%.6f\n", name, uptime());
    fflush(stdout);
    return 0;
}
static int stopped(void) { return access("/run/alibi-tun/stop", F_OK) == 0; }
static void pause_ms(long milliseconds) {
    struct timespec t = { .tv_sec = milliseconds / 1000, .tv_nsec = (milliseconds % 1000) * 1000000L };
    while (nanosleep(&t, &t) != 0 && errno == EINTR) {}
}
static int admitted_guest(void) {
    char hostname[256] = {0}, cmdline[4096] = {0};
    if (geteuid() != 0 || gethostname(hostname, sizeof(hostname)-1) != 0 || strcmp(hostname, "alibi-disposable-guest") != 0) return 0;
    FILE *f = fopen("/proc/cmdline", "r");
    if (!f) return 0;
    size_t n = fread(cmdline, 1, sizeof(cmdline)-1, f); fclose(f);
    if (n == sizeof(cmdline)-1) return 0;
    int run = 0, selected = 0;
    for (char *word = strtok(cmdline, " \t\r\n"); word; word = strtok(NULL, " \t\r\n")) {
        if (strncmp(word, "alibi_run=", 10) == 0 && strlen(word) > 10) run = 1;
        if (strcmp(word, "alibi_case=G13") == 0 || strcmp(word, "alibi_case=G14") == 0) selected = 1;
    }
    return run && selected;
}
int main(int argc, char **argv) {
    if (!admitted_guest()) { fputs("REFUSED: not the admitted TUN guest\n", stderr); return 64; }
    if (argc != 3) return 64;
    char *end1, *end2;
    long emit = strtol(argv[1], &end1, 10), hold = strtol(argv[2], &end2, 10);
    if (*end1 || *end2 || emit < 1 || emit > 20 || hold < 1 || hold > 20) return 64;
    int fd = open("/dev/net/tun", O_RDWR | O_CLOEXEC);
    if (fd < 0) { perror("open guest tun"); return 1; }
    struct ifreq request;
    memset(&request, 0, sizeof(request));
    request.ifr_flags = IFF_TUN | IFF_NO_PI;
    memcpy(request.ifr_name, "aqtun0", sizeof("aqtun0"));
    if (ioctl(fd, TUNSETIFF, &request) != 0) { perror("TUNSETIFF"); close(fd); return 1; }
    printf("TUN_CREATED interface=%s flags=%x uptime=%.6f\n", request.ifr_name, request.ifr_flags, uptime());
    if (phase("READY") != 0) { close(fd); return 1; }
    double deadline = uptime() + 10.0;
    while (access("/run/alibi-tun/go", F_OK) != 0 && !stopped() && uptime() < deadline) pause_ms(10);
    if (access("/run/alibi-tun/go", F_OK) != 0) { close(fd); return 1; }
    if (phase("EMITTING") != 0) { close(fd); return 1; }
    /* Invalid protocol-version nibble. The guest oracle must prove actual rx_dropped. */
    unsigned char bytes[8] = {0};
    unsigned long attempts = 0, accepted = 0, rejected = 0;
    int last_errno = 0;
    deadline = uptime() + (double)emit;
    while (uptime() < deadline && !stopped()) {
        ssize_t result = write(fd, bytes, sizeof(bytes));
        ++attempts;
        if (result < 0) { ++rejected; last_errno = errno; } else ++accepted;
        pause_ms(5);
    }
    printf("TUN_WRITE_COUNTS attempted=%lu accepted=%lu rejected=%lu last_errno=%d uptime=%.6f\n", attempts, accepted, rejected, last_errno, uptime());
    if (phase("HOLD") != 0) { close(fd); return 1; }
    deadline = uptime() + (double)hold;
    while (uptime() < deadline && !stopped()) pause_ms(10);
    close(fd);
    if (phase("FINISHED") != 0) return 1;
    return 0;
}
