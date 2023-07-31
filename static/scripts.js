var sim_chk = document.getElementById("radio1");
var rep_chk = document.getElementById("radio2");
var upl_chk = document.getElementById("radio4");
var conf_chk = document.getElementById("radio5");

showForm("form1");

sim_chk.addEventListener("change", function () {
  if (sim_chk.checked) {
    showForm("form1");
    button1.innerText = "Simulate !";
    var divsToHide = document.getElementsByClassName("sim_elements");
    for (var i = 0; i < divsToHide.length; i++) {
      divsToHide[i].style.display = "block";
    }
    var replayslct = document.getElementsByClassName("replayslct");
    for (var i = 0; i < replayslct.length; i++) {
      replayslct[i].style.display = "none";
    }
  }
});

rep_chk.addEventListener("change", function () {
  if (rep_chk.checked) {
    showForm("form1");
    button1.innerText = "Replay !";
    var divsToHide = document.getElementsByClassName("sim_elements");
    for (var i = 0; i < divsToHide.length; i++) {
      divsToHide[i].style.display = "none";
    }
    var divsToHide = document.getElementsByClassName("replayslct");
    for (var i = 0; i < divsToHide.length; i++) {
      divsToHide[i].style.display = "block";
    }
    var divsToHide = document.getElementsByClassName("replayslct1");
    for (var i = 0; i < divsToHide.length; i++) {
      divsToHide[i].style.display = "none";
    }
    var replayslct = document.getElementsByClassName("replayslct2");
    for (var i = 0; i < replayslct.length; i++) {
      replayslct[i].style.display = "block";
    }
  }
});

upl_chk.addEventListener("change", function () {
  if (upl_chk.checked) {
    showForm("form2");
  }
});

conf_chk.addEventListener("change", function () {
  if (conf_chk.checked) {
    showForm("form3");
  }
});

const actualBtn = document.getElementById("actual-btn");

const fileChosen = document.getElementById("file-chosen");

actualBtn.addEventListener("change", function () {
  fileChosen.textContent = this.files[0].name;
});

function submitConf() {
  // Get form values
  var wd = document.getElementById("wd").value;
  var shutdown = document.getElementById("shutdown").value;
  var imei = document.getElementById("imei").value;
  var threads = document.getElementById("threads").value;

  // Create JSON object
  var data = {
    wd: wd,
    shutdown: shutdown,
    imei: imei,
    threads: threads,
  };

  // Send JSON data to server
  fetch("/config", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
    .then(function (response) {
      // Handle response
      if (response.ok) {
        window.location.href = "/Configured%20Successfuly%20!";
      } else {
        throw new Error("Error: " + response.status);
      }
    })
    .catch(function (error) {
      // Handle error
      console.log("Error:", error);
    });
}

function showForm(formId) {
  // Hide all forms
  var forms = document.getElementsByTagName("form");
  for (var i = 0; i < forms.length; i++) {
    if (forms[i].id !== "except-form") {
      forms[i].style.display = "none";
    }
  }

  // Show the selected form
  var selectedForm = document.getElementById(formId);
  selectedForm.style.display = "block";
}

var addcustom = document.getElementById("addcustom");

var index = 0;

var field_array = [];

//var printVariable = function () {
//  console.log(JSON.stringify(field_array));
//};

//setInterval(printVariable, 1000);

addcustom.addEventListener("click", function () {
  var json_object = { id: index };
  index += 1;
  var fieldDiv = document.getElementById("custom-fields");
  var div = document.createElement("div");
  div.setAttribute("id", "field");
  var label = document.createElement("label");
  label.innerText = "Field Name";
  var input = document.createElement("input");
  input.placeholder = "EnterField Name";
  input.type = "text";
  input.name = "field_name_" + index;
  input.required = true;
  input.addEventListener("change", function (e) {
    if (e.target.value != "") {
      json_object.field_name = e.target.value;
    } else {
      json_object.field_name = null;
    }
    document.getElementById("fields_data").value = JSON.stringify(field_array);
  });

  var divselects = document.createElement("div");
  divselects.setAttribute("id", "http-div");
  divselects.setAttribute("class", "content");

  var labelT = document.createElement("label");
  labelT.innerText = "Select a Type";
  var select = document.createElement("select");
  select.name = "type_" + index;
  select.required = true;
  var option0 = document.createElement("option");
  var option1 = document.createElement("option");
  var option2 = document.createElement("option");
  var option3 = document.createElement("option");
  option0.textContent = "Choose a type";
  option0.value = "";
  option1.textContent = "Boolean";
  option1.value = "bool";
  option2.textContent = "Integer";
  option2.value = "int";
  option3.textContent = "String";
  option3.value = "string";
  select.append(option0, option1, option2, option3);

  var li_div = document.createElement("div");
  li_div.setAttribute("id", "li_div");

  select.addEventListener("change", function (e) {
    switch (e.target.value) {
      case "":
        json_object.type = null;
        if (divselects.childNodes[3]) {
          if (divselects.childNodes[4]) {
            divselects.childNodes[4].remove();
            li_div.innerHTML = "";
          }
          divselects.childNodes[3].remove();
          divselects.childNodes[2].remove();
        }
        document.getElementById("fields_data").value =
          JSON.stringify(field_array);
        break;
      case "bool":
        json_object.type = { bool: null };
        if (divselects.childNodes[3]) {
          if (divselects.childNodes[4]) {
            divselects.childNodes[4].remove();
            li_div.innerHTML = "";
          }
          divselects.childNodes[3].remove();
          divselects.childNodes[2].remove();
        }
        document.getElementById("fields_data").value =
          JSON.stringify(field_array);
        var labelV = document.createElement("label");
        labelV.innerText = "Select a Value";
        var select2 = document.createElement("select");
        select2.required = true;
        select2.name = "bool_option_" + index;

        var opt0 = document.createElement("option");
        var opt1 = document.createElement("option");
        var opt2 = document.createElement("option");
        var opt3 = document.createElement("option");
        opt0.textContent = "Choose a value";
        opt0.value = "";
        opt1.textContent = "True";
        opt1.value = "true";
        opt2.textContent = "False";
        opt2.value = "false";
        opt3.textContent = "Random";
        opt3.value = "random";
        select2.addEventListener("change", function (e) {
          if (e.target.value != "") {
            json_object.type.bool = e.target.value;
          } else {
            json_object.type.bool = null;
          }
          document.getElementById("fields_data").value =
            JSON.stringify(field_array);
        });
        select2.append(opt0, opt1, opt2, opt3);
        divselects.append(labelV, select2);
        break;
      case "int":
        json_object.type = { int: { min: null, max: null, deviation: null } };
        if (divselects.childNodes[3]) {
          if (divselects.childNodes[4]) {
            divselects.childNodes[4].remove();
            li_div.innerHTML = "";
          }
          divselects.childNodes[3].remove();
          divselects.childNodes[2].remove();
        }
        var labelV = document.createElement("label");
        labelV.innerText = "Type Values";
        var divv = document.createElement("div");
        divv.setAttribute("id", "nums");
        var input1 = document.createElement("input");
        var input2 = document.createElement("input");
        var input3 = document.createElement("input");
        input1.placeholder = "Min";
        input2.placeholder = "Max";
        input3.placeholder = "Deviation";
        input1.name = "min_" + index;
        input2.name = "max_" + index;
        input3.name = "deviation_" + index;
        document.getElementById("fields_data").value =
          JSON.stringify(field_array);

        input1.addEventListener("change", function (e) {
          if (e.target.value != "") {
            json_object.type.int = {
              min: parseInt(e.target.value),
              max: json_object.type.int.max,
              deviation: json_object.type.int.deviation,
            };
          } else {
            json_object.type.int.min = null;
          }
          document.getElementById("fields_data").value =
            JSON.stringify(field_array);
        });
        input2.addEventListener("change", function (e) {
          if (e.target.value != "") {
            json_object.type.int = {
              max: parseInt(e.target.value),
              min: json_object.type.int.min,
              deviation: json_object.type.int.deviation,
            };
          } else {
            json_object.type.int.max = null;
          }
          document.getElementById("fields_data").value =
            JSON.stringify(field_array);
        });
        input3.addEventListener("change", function (e) {
          if (e.target.value != "") {
            json_object.type.int = {
              deviation: parseInt(e.target.value),
              max: json_object.type.int.max,
              min: json_object.type.int.min,
            };
          } else {
            json_object.type.int.deviation = null;
          }
          document.getElementById("fields_data").value =
            JSON.stringify(field_array);
        });

        divv.append(input1, input2, input3);
        divselects.append(labelV, divv);

        setInputFilter(
          input1,
          function (value) {
            return (
              /^\d*$/.test(value) && (value === "" || parseInt(value) <= 32767)
            );
          },
          "Must be between 0 and 32767"
        );
        setInputFilter(
          input2,
          function (value) {
            return (
              /^\d*$/.test(value) && (value === "" || parseInt(value) <= 32767)
            );
          },
          "Must be between 0 and 32767"
        );
        setInputFilter(
          input3,
          function (value) {
            return (
              /^\d*$/.test(value) && (value === "" || parseInt(value) <= 32767)
            );
          },
          "Must be between 0 and 32767"
        );

        break;
      case "string":
        json_object.type = { string: null };
        if (divselects.childNodes[3]) {
          if (divselects.childNodes[4]) {
            divselects.childNodes[4].remove();
            li_div.innerHTML = "";
          }
          divselects.childNodes[3].remove();
          divselects.childNodes[2].remove();
        }

        var labelV = document.createElement("label");
        labelV.innerText = "Select a Value";
        var select2 = document.createElement("select");
        select2.required = true;
        select2.name = "str_option_" + index;
        var opt0 = document.createElement("option");
        var opt1 = document.createElement("option");
        var opt2 = document.createElement("option");
        opt0.textContent = "Choose a value";
        opt0.value = "";
        opt1.textContent = "Random value / Fixed size";
        opt1.value = "random";
        opt2.textContent = "Predefined Values";
        opt2.value = "array";
        select2.append(opt0, opt1, opt2);

        select2.addEventListener("change", function (e) {
          switch (e.target.value) {
            case "":
              json_object.type.string = null;
              if (divselects.childNodes[4]) {
                divselects.childNodes[4].remove();
                li_div.innerHTML = "";
              }
              document.getElementById("fields_data").value =
                JSON.stringify(field_array);
              break;
            case "random":
              if (divselects.childNodes[4]) {
                divselects.childNodes[4].remove();
                li_div.innerHTML = "";
              }
              json_object.type.string = { random: null };
              var labelS = document.createElement("label");
              labelS.innerText = "Select a Size";
              var size = document.createElement("input");
              size.placeholder = "Type a size";
              size.name = "size_" + index;

              size.addEventListener("change", function (e) {
                if (e.target.value != "") {
                  json_object.type.string.random = parseInt(e.target.value);
                } else {
                  json_object.type.string.random = null;
                }
                document.getElementById("fields_data").value =
                  JSON.stringify(field_array);
              });

              setInputFilter(
                size,
                function (value) {
                  return (
                    /^\d*$/.test(value) &&
                    (value === "" || parseInt(value) <= 10)
                  );
                },
                "Must be between 0 and 10"
              );

              var divv = document.createElement("div");
              divv.setAttribute("id", "nums");
              divv.append(labelS, size);
              divselects.append(divv);
              break;
            case "array":
              if (divselects.childNodes[4]) {
                divselects.childNodes[4].remove();
                li_div.innerHTML = "";
              }
              json_object.type.string = { array: [] };
              var labelS = document.createElement("label");
              labelS.innerText = "Type your Strings (Ctrl+ENTER to add)";
              var nb_strings_input = document.createElement("input");
              nb_strings_input.name = "nbr_strings_" + index;
              nb_strings_input.hidden = true;
              var string = document.createElement("input");
              string.placeholder = "Type a String";
              var nbe_strings = 0;
              var ul = document.createElement("ul");
              li_div.append(ul, nb_strings_input);
              string.addEventListener("keyup", function (e) {
                if (e.key === "Enter" || e.keyCode === 13) {
                  // Prevent the default action of submitting the form
                  e.preventDefault();
                  if (string.value != "") {
                    nbe_strings += 1;
                    var imput = document.createElement("input");
                    imput.name = "string_option_" + index + "_" + nbe_strings;
                    imput.hidden = true;
                    imput.value = string.value;
                    json_object.type.string.array.push(string.value);
                    document.getElementById("fields_data").value =
                      JSON.stringify(field_array);
                    var li = document.createElement("li");
                    li.innerText = string.value;
                    ul.append(li);
                    string.value = "";
                    nb_strings_input.value = nbe_strings;
                    li_div.append(imput);
                  }
                }
              });
              var divv = document.createElement("div");
              divv.setAttribute("id", "array");
              divv.append(labelS, string);
              divselects.append(divv);
              break;
          }
        });

        divselects.append(labelV, select2);

        break;
    }
  });

  divselects.append(labelT, select);

  var labelP = document.createElement("label");
  labelP.innerText =
    "Cyclic Time Period in seconds (Simulate every {?} / Pattern : %xh%xm%xs (%x is a number)";
  var inputP = document.createElement("input");
  inputP.placeholder = "Enter Time Period (seconds)";
  inputP.type = "text";
  inputP.required = true;
  inputP.setAttribute("id", "freq_input");
  inputP.name = "frequence_" + index;

  inputP.addEventListener("change", function (e) {
    if (/^\d+h\d+m\d+s$/.test(e.target.value)) {
      json_object.frequence = e.target.value;
      inputP.setCustomValidity("");
    } else {
      inputP.setCustomValidity("Must follow the pattern %xh%xm%xs");
      inputP.reportValidity();
      json_object.frequence = null;
    }
    document.getElementById("fields_data").value = JSON.stringify(field_array);
  });

  var deleteB = document.createElement("button");
  deleteB.setAttribute("class", "btn btn-danger");
  deleteB.addEventListener("click", function (e) {
    e.preventDefault();

    const indexToDelete = field_array.findIndex(
      (obj) => obj.id === json_object.id
    );

    if (indexToDelete !== -1) {
      field_array.splice(indexToDelete, 1);
    }

    index--;
    document.getElementById("fields_data").value = JSON.stringify(field_array);
    div.remove();
  });
  deleteB.innerText = "Delete";

  var br1 = document.createElement("br");
  var br2 = document.createElement("br");

  div.append(
    label,
    br1,
    input,
    divselects,
    li_div,
    labelP,
    br2,
    inputP,
    deleteB
  );
  fieldDiv.appendChild(div);
  document.getElementById("fields_size").value = index;
  field_array.push(json_object);
});

function setInputFilter(textbox, inputFilter, errMsg) {
  [
    "input",
    "keydown",
    "keyup",
    "mousedown",
    "mouseup",
    "select",
    "contextmenu",
    "drop",
    "focusout",
  ].forEach(function (event) {
    textbox.addEventListener(event, function (e) {
      if (inputFilter(this.value)) {
        // Accepted value
        if (["keydown", "mousedown", "focusout"].indexOf(e.type) >= 0) {
          this.classList.remove("input-error");
          this.setCustomValidity("");
        }
        this.oldValue = this.value;
        this.oldSelectionStart = this.selectionStart;
        this.oldSelectionEnd = this.selectionEnd;
      } else if (this.hasOwnProperty("oldValue")) {
        this.classList.add("input-error");
        this.setCustomValidity(errMsg);
        this.reportValidity();
        this.value = this.oldValue;
        this.setSelectionRange(this.oldSelectionStart, this.oldSelectionEnd);
      } else {
        this.value = "";
      }
    });
  });
}

function openPokeForm(threadUrl) {
  const pokeFormOverlay = document.getElementById("pokeFormOverlay");
  const pokeForm = document.getElementById("pokeForm");
  const pokeMessageInput = document.getElementById("pokeMessage");

  pokeFormOverlay.classList.add("active");
  pokeForm.classList.add("active");
  pokeMessageInput.focus();

  pokeForm.dataset.url = threadUrl;

  pokeFormOverlay.addEventListener("click", closePokeFormOverlay);
}

function closePokeFormOverlay() {
  const pokeFormOverlay = document.getElementById("pokeFormOverlay");
  const pokeForm = document.getElementById("pokeForm");
  const pokeMessageInput = document.getElementById("pokeMessage");

  pokeFormOverlay.classList.remove("active");
  pokeForm.classList.remove("active");
  pokeMessageInput.value = "";

  pokeFormOverlay.removeEventListener("click", closePokeFormOverlay);
}

function sendPoke(event) {
  event.preventDefault();
  const threadUrl = pokeForm.dataset.url;
  const PokeId = document.getElementById("PokeId").value;
  const PokeImei = document.getElementById("PokeImei").value;
  const PokeSender = document.getElementById("PokeSender").value;
  const PokeNamespace = document.getElementById("PokeNamespace").value;
  const pokeMessage = document.getElementById("pokeMessage").value;
  const currentDate = new Date();
  const formattedDate = currentDate.toISOString();

  const jsonString = JSON.stringify([
    {
      meta: {
        account: "municio",
        event: "poke",
      },
      payload: {
        id: parseInt(PokeId),
        id_str: PokeId,
        asset: PokeImei,
        sender: PokeSender,
        namespace: PokeNamespace,
        received_at: formattedDate,
        b64_message: pokeMessage,
      },
    },
  ]);

  console.log(`Sending poke to ${threadUrl} with message: ${jsonString}`);

  fetch(threadUrl, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: jsonString,
  }).then((response) => {
    if (response.ok) {
      console.log("Poke sent successfully!");
    } else {
      console.log("Failed to send poke.");
    }
  });

  closePokeFormOverlay();
}
