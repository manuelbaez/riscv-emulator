pub enum OperationSideEffect {
    None,
    SkipPCIncrease,
    TriggerSyscall,
    TriggerBreakpoint,
}
