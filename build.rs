use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header("/usr/include/linux/aio_abi.h")
        .derive_default(true)
        .whitelist_type("aio_context_t")
        .whitelist_type("iocb")
        .whitelist_type("io_event")
        .whitelist_var("IOCB_CMD_PREAD")
        .whitelist_var("IOCB_CMD_PWRITE")
        .whitelist_var("IOCB_CMD_FSYNC")
        .whitelist_var("IOCB_CMD_FDSYNC")
        .whitelist_var("IOCB_CMD_POLL")
        .whitelist_var("IOCB_CMD_NOOP")
        .whitelist_var("IOCB_CMD_PREADV")
        .whitelist_var("IOCB_CMD_PWRITEV")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
