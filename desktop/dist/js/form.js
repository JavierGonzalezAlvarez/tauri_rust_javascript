function formFunction(form) {
    const invoke_form = window.__TAURI__.invoke

    //Get values from the Form
    nm = form.elements["name"].value;
    phone = form.elements["phone"].value;
    email = form.elements["email"].value;
    comments = form.elements["comments"].value;
    console.log(nm, "-", phone, "-", email, "-", comments);

    //create a FormData
    const myFormData = new FormData(form);
    var data = {};
    myFormData.forEach(function (value, key) {
        data[key] = value;
    });
    //convert to json
    var datajson = JSON.stringify(data);
    console.log(datajson);

    //send data to back
    invoke_form('form_buttom_command', { data: datajson })
}