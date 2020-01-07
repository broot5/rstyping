function draw() {
  let ctx = document.getElementById("chart");
  let chart = new Chart(ctx, {
    type: "line",
    data: {
      labels: ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
      datasets: [
        {
          label: "Typing speed",
          yAxisID: "A",
          data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        },
        {
          label: "Accuracy",
          yAxisID: "B",
          data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        }
      ]
    },
    options: {
      scales: {
        yAxes: [
          {
            id: "A",
            type: "linear",
            position: "left",
            ticks: {
              max: 1000,
              min: 0
            }
          },
          {
            id: "B",
            type: "linear",
            position: "right",
            ticks: {
              max: 100,
              min: 0
            }
          }
        ]
      }
    }
  });
}

let accuracyData = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
let typingSpeedData = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

window.onload = function() {
  draw();
};

console.log(chart.data.datasets[0].data);

setInterval(function(chart) {
  let tempArray = document.getElementById("result").innerText.split(" ");

  let accuracy = parseInt(tempArray[0].slice(0));
  let typingSpeed = parseInt(tempArray[1]);

  accuracyData.shift();
  typingSpeedData.shift();

  accuracyData.push(accuracy);
  typingSpeedData.push(typingSpeed);

  redrawChart(chart, accuracyData, typingSpeedData);
}, 1000);

function redrawChart(chart, accuracyData, typingSpeedData) {
  chart.data.datasets[0].data = typingSpeedData;
  chart.data.datasets[1].data = accuracyData;

  chart.update();
}
