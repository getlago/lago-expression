import { parseExpression, evaluateExpression } from "./pkg";

var elem = document.getElementById("expression");
var output = document.getElementById("expression-output");
var error = document.getElementById("expression-error");

var expression = null;

elem.oninput = function () {
  error.innerHTML = "";
  try {
    expression = parseExpression(elem.value);
    output.innerHTML = evaluateExpression(expression, "code", 1231254123, {
      started_at: 123124123,
      ended_at: 1241231241,
      replicas: 8,
    });
  } catch (e) {
    error.innerHTML = e;
  }
};
