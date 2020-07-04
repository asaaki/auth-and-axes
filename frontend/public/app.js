// https://stackoverflow.com/a/59234388/653173
function serializeForm(form) {
  return Array.from(new FormData(form).entries()).reduce(
    (data, [field, value]) => {
      let [_, prefix, keys] = field.match(/^([^\[]+)((?:\[[^\]]*\])*)/);

      if (keys) {
        keys = Array.from(keys.matchAll(/\[([^\]]*)\]/g), (m) => m[1]);
        value = update(data[prefix], keys, value);
      }
      data[prefix] = value;
      return data;
    },
    {}
  );
}

function update(data, keys, value) {
  if (keys.length === 0) {
    // Leaf node
    return value;
  }

  let key = keys.shift();
  if (!key) {
    data = data || [];
    if (Array.isArray(data)) {
      key = data.length;
    }
  }

  // Try converting key to a numeric value
  let index = +key;
  if (!isNaN(index)) {
    // We have a numeric index, make data a numeric array
    // This will not work if this is a associative array
    // with numeric keys
    data = data || [];
    key = index;
  }

  // If none of the above matched, we have an associative array
  data = data || {};

  let val = update(data[key], keys, value);
  data[key] = val;

  return data;
}

async function postData(url = "", data = {}) {
  // Default options are marked with *
  const response = await fetch(url, {
    method: "POST", // *GET, POST, PUT, DELETE, etc.
    mode: "cors", // no-cors, *cors, same-origin
    cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
    credentials: "include", // include, *same-origin, omit
    headers: {
      "content-type": "application/json",
    },
    redirect: "follow", // manual, *follow, error
    referrerPolicy: "strict-origin-when-cross-origin", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
    body: JSON.stringify(data), // body data type must match "Content-Type" header
  });
  return response.json(); // parses JSON response into native JavaScript objects
}

function responseHandler(response) {
  console.log("received:", response);

  let output = document.getElementById("loginResponse");
  output.innerText = JSON.stringify(response, undefined, 2);
}

function processForm(e) {
  if (e.preventDefault) e.preventDefault();
  let payload = serializeForm(e.target);
  console.log("json string", JSON.stringify(payload));
  postData("https://id.unicorn.test:8099/login", payload).then(
    responseHandler
  );
  return false;
}

let formElement = document.getElementById("loginForm");
if(formElement) {
  if (formElement.attachEvent) {
    formElement.attachEvent("submit", processForm);
  } else {
    formElement.addEventListener("submit", processForm);
  }
}

console.log("app started");
