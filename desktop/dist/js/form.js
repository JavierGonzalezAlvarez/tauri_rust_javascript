function formFunction() {
    const invoke_form = window.__TAURI__.invoke
    //Invoke the command
    invoke_form('form_buttom_command')
}