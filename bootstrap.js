'use strict';

import Chartjs from 'chart.js';

main();

async function main() {
  await import('./pkg').then(module => {
    module.run_app();
  });

  function parseData() {
    const tempArray = document.querySelector('#result').innerText.split(' ');

    let accuracy = parseInt(tempArray[0].slice(0), 10);
    let typingSpeed = parseInt(tempArray[1], 10);

    if (Number.isNaN(accuracy) || Number.isNaN(typingSpeed)) {
      accuracy = 0;
      typingSpeed = 0;
    }

    return { accuracy, typingSpeed };
  }

  class Chart {
    constructor(ctx) {
      this.ctx = ctx;
      this.chart = new Chartjs(this.ctx, {
        type: 'line',
        data: {
          labels: ['1', '2', '3', '4', '5', '6', '7', '8', '9', '10'],
          datasets: [
            {
              label: 'Typing speed',
              yAxisID: 'A',
              data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            },
            {
              label: 'Accuracy',
              yAxisID: 'B',
              data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            },
          ],
        },
        options: {
          scales: {
            yAxes: [
              {
                id: 'A',
                type: 'linear',
                position: 'left',
                ticks: {
                  max: 1000,
                  min: 0,
                },
              },
              {
                id: 'B',
                type: 'linear',
                position: 'right',
                ticks: {
                  max: 100,
                  min: 0,
                },
              },
            ],
          },
        },
      });
      this.accuracyData = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
      this.typingSpeedData = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    }

    update() {
      const { accuracy, typingSpeed } = parseData();

      this.accuracyData.shift();
      this.typingSpeedData.shift();

      this.accuracyData.push(accuracy);
      this.typingSpeedData.push(typingSpeed);

      this.updateChart();
    }

    updateChart() {
      this.chart.data.datasets[0].data = this.typingSpeedData;
      this.chart.data.datasets[1].data = this.accuracyData;

      this.chart.update();
    }
  }

  const mainChart = new Chart(document.querySelector('#chart'));

  window.addEventListener(
    'keypress',
    function(event) {
      if (event.code == 'Enter') {
        mainChart.update();
      }
    },
    true
  );
}
