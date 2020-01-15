import Chart from "chart.js";

main();

async function main() {
  await import("./pkg").then(module => {
    module.run_app();
  });

  let ctx = document.getElementById("chart");

  let aChart = new Chart(ctx, {
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

  let accuracyData = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  let typingSpeedData = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

  updateData(aChart, accuracyData, typingSpeedData); //For test; call updateData on js available, this line has to remove
}

function updateData(chart, accuracyData, typingSpeedData) {
  let tempArray = document.getElementById("result").innerText.split(" ");

  let accuracy = parseInt(tempArray[0].slice(0));
  let typingSpeed = parseInt(tempArray[1]);

  if (isNaN(accuracy) || isNaN(typingSpeed)) {
    accuracy = 0;
    typingSpeed = 0;
  }

  accuracyData.shift();
  typingSpeedData.shift();

  accuracyData.push(accuracy);
  typingSpeedData.push(typingSpeed);

  redrawChart(chart, accuracyData, typingSpeedData);
}

function redrawChart(chart, accuracyData, typingSpeedData) {
  chart.data.datasets[0].data = typingSpeedData;
  chart.data.datasets[1].data = accuracyData;

  chart.update();
}
