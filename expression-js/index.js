import { parseExpression, evaluateExpression } from "./pkg";

var elem = document.getElementById("expression");
var output = document.getElementById("expression-output");
var error = document.getElementById("expression-error");

var expression = null;

elem.oninput = function () {
  error.innerHTML = "";
  try {
    expression = parseExpression(elem.value);
    output.innerHTML = evaluateExpression(expression, "code", BigInt(1231254123), {
      started_at: BigInt(123124123),
      ended_at: BigInt(1241231241),
      replicas: BigInt(8),
    });
  } catch (e) {
    error.innerHTML = e;
  }
};
