// This file is autogenerated. Do not edit by hand.
// To regenerate from schema, run `cargo run -p generator`.

pub use crate::IEvent;

/// The event indicates that some information about a breakpoint has changed.
///
/// See [BreakpointEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Breakpoint)
pub enum Breakpoint {}

impl IEvent for Breakpoint {
    const EVENT: &'static str = "breakpoint";
    type Body = crate::BreakpointEvent;
}

/// The event indicates that one or more capabilities have changed.
/// Since the capabilities are dependent on the client and its UI, it might not be possible to change that at random times (or too late).
/// Consequently this event has a hint characteristic: a client can only be expected to make a 'best effort' in honoring individual capabilities but there are no guarantees.
/// Only changed capabilities need to be included, all other capabilities keep their values.
///
/// See [CapabilitiesEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Capabilities)
pub enum Capabilities {}

impl IEvent for Capabilities {
    const EVENT: &'static str = "capabilities";
    type Body = crate::CapabilitiesEvent;
}

/// The event indicates that the execution of the debuggee has continued.
/// Please note: a debug adapter is not expected to send this event in response to a request that implies that execution continues, e.g. `launch` or `continue`.
/// It is only necessary to send a `continued` event if there was no previous request that implied this.
///
/// See [ContinuedEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Continued)
pub enum Continued {}

impl IEvent for Continued {
    const EVENT: &'static str = "continued";
    type Body = crate::ContinuedEvent;
}

/// The event indicates that the debuggee has exited and returns its exit code.
///
/// See [ExitedEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Exited)
pub enum Exited {}

impl IEvent for Exited {
    const EVENT: &'static str = "exited";
    type Body = crate::ExitedEvent;
}

/// This event indicates that the debug adapter is ready to accept configuration requests (e.g. `setBreakpoints`, `setExceptionBreakpoints`).
/// A debug adapter is expected to send this event when it is ready to accept configuration requests (but not before the `initialize` request has finished).
/// The sequence of events/requests is as follows:
/// - adapters sends `initialized` event (after the `initialize` request has returned)
/// - client sends zero or more `setBreakpoints` requests
/// - client sends one `setFunctionBreakpoints` request (if corresponding capability `supportsFunctionBreakpoints` is true)
/// - client sends a `setExceptionBreakpoints` request if one or more `exceptionBreakpointFilters` have been defined (or if `supportsConfigurationDoneRequest` is not true)
/// - client sends other future configuration requests
/// - client sends one `configurationDone` request to indicate the end of the configuration.
///
/// See [InitializedEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Initialized)
pub enum Initialized {}

impl IEvent for Initialized {
    const EVENT: &'static str = "initialized";
    type Body = Option<crate::Capabilities>;
}

/// This event signals that some state in the debug adapter has changed and requires that the client needs to re-render the data snapshot previously requested.
/// Debug adapters do not have to emit this event for runtime changes like stopped or thread events because in that case the client refetches the new state anyway. But the event can be used for example to refresh the UI after rendering formatting has changed in the debug adapter.
/// This event should only be sent if the corresponding capability `supportsInvalidatedEvent` is true.
///
/// See [InvalidatedEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Invalidated)
pub enum Invalidated {}

impl IEvent for Invalidated {
    const EVENT: &'static str = "invalidated";
    type Body = crate::InvalidatedEvent;
}

/// The event indicates that some source has been added, changed, or removed from the set of all loaded sources.
///
/// See [LoadedSourceEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_LoadedSource)
pub enum LoadedSource {}

impl IEvent for LoadedSource {
    const EVENT: &'static str = "loadedSource";
    type Body = crate::LoadedSourceEvent;
}

/// This event indicates that some memory range has been updated. It should only be sent if the corresponding capability `supportsMemoryEvent` is true.
/// Clients typically react to the event by re-issuing a `readMemory` request if they show the memory identified by the `memoryReference` and if the updated memory range overlaps the displayed range. Clients should not make assumptions how individual memory references relate to each other, so they should not assume that they are part of a single continuous address range and might overlap.
/// Debug adapters can use this event to indicate that the contents of a memory range has changed due to some other request like `setVariable` or `setExpression`. Debug adapters are not expected to emit this event for each and every memory change of a running program, because that information is typically not available from debuggers and it would flood clients with too many events.
///
/// See [MemoryEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Memory)
pub enum Memory {}

impl IEvent for Memory {
    const EVENT: &'static str = "memory";
    type Body = crate::MemoryEvent;
}

/// The event indicates that some information about a module has changed.
///
/// See [ModuleEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Module)
pub enum Module {}

impl IEvent for Module {
    const EVENT: &'static str = "module";
    type Body = crate::ModuleEvent;
}

/// The event indicates that the target has produced some output.
///
/// See [OutputEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Output)
pub enum Output {}

impl IEvent for Output {
    const EVENT: &'static str = "output";
    type Body = crate::OutputEvent;
}

/// The event indicates that the debugger has begun debugging a new process. Either one that it has launched, or one that it has attached to.
///
/// See [ProcessEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Process)
pub enum Process {}

impl IEvent for Process {
    const EVENT: &'static str = "process";
    type Body = crate::ProcessEvent;
}

/// The event signals the end of the progress reporting with a final message.
/// This event should only be sent if the corresponding capability `supportsProgressReporting` is true.
///
/// See [ProgressEndEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_ProgressEnd)
pub enum ProgressEnd {}

impl IEvent for ProgressEnd {
    const EVENT: &'static str = "progressEnd";
    type Body = crate::ProgressEndEvent;
}

/// The event signals that a long running operation is about to start and provides additional information for the client to set up a corresponding progress and cancellation UI.
/// The client is free to delay the showing of the UI in order to reduce flicker.
/// This event should only be sent if the corresponding capability `supportsProgressReporting` is true.
///
/// See [ProgressStartEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_ProgressStart)
pub enum ProgressStart {}

impl IEvent for ProgressStart {
    const EVENT: &'static str = "progressStart";
    type Body = crate::ProgressStartEvent;
}

/// The event signals that the progress reporting needs to be updated with a new message and/or percentage.
/// The client does not have to update the UI immediately, but the clients needs to keep track of the message and/or percentage values.
/// This event should only be sent if the corresponding capability `supportsProgressReporting` is true.
///
/// See [ProgressUpdateEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_ProgressUpdate)
pub enum ProgressUpdate {}

impl IEvent for ProgressUpdate {
    const EVENT: &'static str = "progressUpdate";
    type Body = crate::ProgressUpdateEvent;
}

/// The event indicates that the execution of the debuggee has stopped due to some condition.
/// This can be caused by a breakpoint previously set, a stepping request has completed, by executing a debugger statement etc.
///
/// See [StoppedEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Stopped)
pub enum Stopped {}

impl IEvent for Stopped {
    const EVENT: &'static str = "stopped";
    type Body = crate::StoppedEvent;
}

/// The event indicates that debugging of the debuggee has terminated. This does **not** mean that the debuggee itself has exited.
///
/// See [TerminatedEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Terminated)
pub enum Terminated {}

impl IEvent for Terminated {
    const EVENT: &'static str = "terminated";
    type Body = crate::TerminatedEvent;
}

/// The event indicates that a thread has started or exited.
///
/// See [ThreadEvent](https://microsoft.github.io/debug-adapter-protocol/specification#Events_Thread)
pub enum Thread {}

impl IEvent for Thread {
    const EVENT: &'static str = "thread";
    type Body = crate::ThreadEvent;
}
