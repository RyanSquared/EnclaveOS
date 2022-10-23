use system::{seed_entropy, reboot, freopen, mount, dmesg};

//TODO: Feature flag
use aws::{init_platform, get_entropy};

// Mount common filesystems with conservative permissions
fn init_rootfs() {
    use libc::{MS_NOSUID, MS_NOEXEC, MS_NODEV };
    let no_dse = MS_NODEV | MS_NOSUID | MS_NOEXEC;
    let no_se = MS_NOSUID | MS_NOEXEC;
    let args = [
        ("devtmpfs",    "/dev",           "devtmpfs", no_se,  "mode=0755"),
        ("devtmpfs",    "/dev",           "devtmpfs", no_se,  "mode=0755"),
        ("devpts",      "/dev/pts",       "devpts",   no_se,  ""),
        ("shm",         "/dev/shm",       "tmpfs",    no_dse, "mode=0755"),
        ("proc",        "/proc",          "proc",     no_dse, "hidepid=2"),
        ("tmpfs",       "/run",           "tmpfs",    no_dse, "mode=0755"),
        ("tmpfs",       "/tmp",           "tmpfs",    no_dse, ""),
        ("sysfs",       "/sys",           "sysfs",    no_dse, ""),
        ("cgroup_root", "/sys/fs/cgroup", "tmpfs",    no_dse, "mode=0755"),
    ];
    for (src, target, fstype, flags, data) in args {
        match mount(src, target, fstype, flags, data) {
            Ok(())=> dmesg(format!("Mounted {}", target)),
            Err(e)=> eprintln!("{}", e),
        }
    }
}

// Initialize console with stdin/stdout/stderr
fn init_console() {
    let args = [
        ("/dev/console", "r", 0),
        ("/dev/console", "w", 1),
        ("/dev/console", "w", 2),
    ];
    for (filename, mode, file) in args {
        match freopen(filename, mode, file) {
            Ok(())=> {},
            Err(e)=> eprintln!("{}", e),
        }
    }
}

fn boot(){
    init_rootfs();
    init_console();
    init_platform();
    match seed_entropy(4096, get_entropy) {
        Ok(size)=> dmesg(format!("Seeded kernel with entropy: {}", size)),
        Err(e)=> eprintln!("{}", e)
    };
}

fn main() {
    boot();
    dmesg("EnclaveOS Booted".to_string());
    reboot();
}
