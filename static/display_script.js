const url = document.getElementById("url");
const json_file = document.getElementById("json_file");
const button1 = document.getElementById("button");
const upload_button = document.getElementById("upload_button");
const http = document.getElementById("http");
const http_div = document.getElementById("http-div");
const key = document.getElementById("key");
const destination = document.getElementById("destination");
const source = document.getElementById("source");
destination.disabled = true;
source.disabled = true;
button1.disabled = true;
upload_button.disabled = true;
const evtSource = new EventSource("/events");
var table = document.getElementById("thread-table");
table.style.display = "none";
var tf_select = document.getElementById("tf_select");
var t_select = document.getElementById("t_select");
var pf_select = document.getElementById("pf_select");
var p_select = document.getElementById("p_select");
var recordbtn = document.getElementById("record-btn");
recordbtn.hidden = true;
const buttonText = recordbtn.querySelector("span");
var chosen_json_file = document.getElementById("chosen_json_file");

var replayslct = document.getElementsByClassName("replayslct");
for (var i = 0; i < replayslct.length; i++) {
  replayslct[i].style.display = "none";
}

recordbtn.addEventListener("click", (e) => {
  e.preventDefault();
  $.ajax({
    type: "POST",
    url: "/record",
    success: function (result) {
      console.log("success");
    },
    error: function (result) {
      console.log("error");
    },
  });
});

evtSource.onopen = function () {
  console.log("Connection to server opened.");
};

evtSource.onmessage = function (e) {
  const obj = JSON.parse(e.data);
  // console.log(obj)

  if (obj.record == 1) {
    buttonText.textContent = "Stop Recording";
  } else {
    buttonText.textContent = "Record Trips";
  }

  if (table.rows.length == 1) {
    table.style.display = "none";
  } else {
    table.style.display = "block";
  }

  var tbodyRef = document
    .getElementById("thread-table")
    .getElementsByTagName("tbody")[0];

  tbodyRef.innerHTML = "";

  for (const [key, value] of Object.entries(obj.threads)) {
    var newRow = tbodyRef.insertRow();
    newRow.setAttribute("id", "row" + key);

    var newCell = newRow.insertCell();
    newCell.setAttribute("class", "column-data");

    var newText = document.createTextNode(key);

    newCell.appendChild(newText);

    var newCell1 = newRow.insertCell();

    var btn = document.createElement("button");
    btn.setAttribute("class", "inp");
    btn.setAttribute("id", "btn");
    btn.textContent = "Abort Thread !";
    newCell1.appendChild(btn);

    var newCell2 = newRow.insertCell();

    const newSpan = document.createElement("span");
    newSpan.setAttribute("id", "newSpan");
    newSpan.textContent = "●";
    const newP = document.createElement("p");
    newP.setAttribute("id", "http");
    const newDiv = document.createElement("div");
    newDiv.setAttribute("id", "http-div");

    if (value.code == 1) {
      newSpan.setAttribute("class", "logged-in");
      newP.textContent = "code : 200";
    } else if (value.code == 0) {
      newSpan.setAttribute("class", "logged-out");
      newP.textContent = "code : 503";
    }

    var newCellTime = newRow.insertCell();
    const PTime = document.createElement("p");

    var newCell3 = newRow.insertCell();
    const newP1 = document.createElement("p");
    newP1.setAttribute("id", "resp_message");
    if (value.ok_msg == "" && value.err_msg == "") {
      newSpan.setAttribute("class", "waiting");
      newP.textContent = "code : _";
      newP1.textContent = "Waiting for response ...";
      PTime.textContent = "Waiting for response ...";
    } else if (value.code == 0) {
      newSpan.setAttribute("class", "logged-out");
      newP.textContent = "code : 503";
      newP1.textContent = value.err_msg;
      PTime.textContent = value.timestamp;
    } else if (value.code == 1) {
      newSpan.setAttribute("class", "logged-in");
      newP.textContent = "code : 200";
      newP1.textContent = value.ok_msg;
      PTime.textContent = value.timestamp;
    }
    newCell3.appendChild(newP1);
    newCellTime.appendChild(PTime);

    newDiv.appendChild(newSpan);
    newDiv.appendChild(newP);
    newCell2.appendChild(newDiv);
  }

  let allButtons = document.getElementsByClassName("inp");
  for (let button of allButtons) {
    button.addEventListener("click", () => {
      var clickedElement = event.target;
      var clickedRow = clickedElement.parentNode.parentNode.id;
      var rowData = document
        .getElementById(clickedRow)
        .querySelectorAll(".column-data");
      let url = rowData[0].innerHTML;
      e.preventDefault();
      $.ajax({
        type: "POST",
        url: "/abort",
        data: url,
        success: function (result) {
          console.log("success");
        },
        error: function (result) {
          console.log("error");
        },
      });
    });
  }
};

evtSource.onerror = function () {
  console.log("EventSource failed.");
};

key.addEventListener("input", (event) => {
  if (key.value == "") {
    destination.disabled = true;
    source.disabled = true;
  } else {
    source.disabled = false;
    destination.disabled = false;
  }

  button1.disabled =
    (pf_select.value == "" || tf_select.value == "" || url.value == "") &&
    (chosen_json_file.value == "" || url.value == "") &&
    (key.value == "" ||
      url.value == "" ||
      source.value == "" ||
      destination.value == "");
});

source.addEventListener("input", (event) => {
  button1.disabled =
    /\d+h\d+m\d+s/.test() ||
    key.value == "" ||
    url.value == "" ||
    source.value == "" ||
    destination.value == "";
});
destination.addEventListener("input", (event) => {
  button1.disabled =
    key.value == "" ||
    url.value == "" ||
    source.value == "" ||
    destination.value == "";
});

json_file.addEventListener("change", (event) => {
  if (event.target.files[0]) {
    upload_button.disabled = false;
  } else {
    upload_button.disabled = true;
  }
});

tf_select.addEventListener("change", (event) => {
  removeOptions(t_select);
  for (var i = 0; i < tdata.length; i++) {
    if (tdata[i]._id.file == event.target.value) {
      var new_option = document.createElement("option");
      new_option.text = `${tdata[i]._id.year}-${tdata[i]._id.month}-${tdata[i]._id.day}`;
      new_option.value = `${tdata[i]._id.year}-${tdata[i]._id.month}-${tdata[i]._id.day}`;
      t_select.appendChild(new_option);
    }
  }
  button1.disabled =
    (pf_select.value == "" || tf_select.value == "" || url.value == "") &&
    (chosen_json_file.value == "" || url.value == "") &&
    (key.value == "" || url.value == "");
});
pf_select.addEventListener("change", (event) => {
  removeOptions(p_select);
  for (var i = 0; i < pdata.length; i++) {
    if (pdata[i]._id.file == event.target.value) {
      var new_option = document.createElement("option");
      new_option.text = `${pdata[i]._id.year}-${pdata[i]._id.month}-${pdata[i]._id.day}`;
      new_option.value = `${pdata[i]._id.year}-${pdata[i]._id.month}-${pdata[i]._id.day}`;
      p_select.appendChild(new_option);
    }
  }
  button1.disabled =
    (pf_select.value == "" || tf_select.value == "" || url.value == "") &&
    (chosen_json_file.value == "" || url.value == "") &&
    (key.value == "" || url.value == "");
});

chosen_json_file.addEventListener("change", (event) => {
  button1.disabled =
    (pf_select.value == "" || tf_select.value == "" || url.value == "") &&
    (chosen_json_file.value == "" || url.value == "") &&
    (key.value == "" || url.value == "");
});

function removeOptions(selectElement) {
  var i,
    L = selectElement.options.length - 1;
  for (i = L; i >= 0; i--) {
    selectElement.remove(i);
  }
}

url.addEventListener("input", (event) => {
  button1.disabled =
    !(
      tf_select.value != "" &&
      pf_select.value != "" &&
      validator.isURL(event.target.value)
    ) &&
    !(validator.isURL(event.target.value) && chosen_json_file.value != "") &&
    !(validator.isURL(event.target.value) && key.value != "");
});
