// htmx.defineExtension("json-enc", {
//   onEvent: function (name, evt) {
//     if (name === "htmx:configRequest") {
//       evt.detail.headers["Content-Type"] = "application/json";
//     }
//   },

//   encodeParameters: function (xhr, parameters, elt) {
//     xhr.overrideMimeType("text/json");
//     return JSON.stringify(parameters);
//   },
// });

htmx.defineExtension("submitjson", {
  onEvent: function (name, evt) {
    if (name === "htmx:configRequest") {
      evt.detail.headers["Content-Type"] = "application/json";
    }
  },
  encodeParameters: function (xhr, parameters) {
    let json = {};
    for (const key in parameters) {
      const input = document.querySelector(`[name="${key}"]`);
      if (input) {
        if (input.type === "number") {
          json[key] = parseFloat(parameters[key]);
        } else if (input.type === "checkbox") {
          json[key] = input.checked;
        } else {
          json[key] = parameters[key];
        }
      }
    }

    console.log(json);
    return JSON.stringify(json);
  },
});
