function rustFunction() {
    const invoke_rust = window.__TAURI__.invoke
    //Invoke the command
    invoke_rust('rust_buttom_command')
}