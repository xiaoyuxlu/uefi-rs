/// This function is generated
#[no_mangle]
#[export_name = "__imp_RtlVirtualUnwind"]
pub extern "C" fn RtlVirtualUnwind() {
    panic!("RtlVirtualUnwind called");
}

#[no_mangle]
pub unsafe extern "C" fn ___chkstk_ms() {
    asm!(
        "push %rcx",
        "push %rax",
        "cmp $0x1000,%rax",
        "lea 24(%rsp),%rcx",
        "jb 1f",
        "2:",
        "sub $0x1000,%rcx",
        "test %rcx,(%rcx)",
        "sub $0x1000,%rax",
        "cmp $0x1000,%rax",
        "ja 2b",
        "1:",
        "sub %rax,%rcx",
        "test %rcx,(%rcx)",
        "pop %rax",
        "pop %rcx",
        "ret",
        options(noreturn, att_syntax)
    );
}
