<!--section-start::header-->  <head>
    <meta charset="utf-8">
    <title>{{title}}</title>
    {{>scripts}}
  </head><!--section-end::header-->

<!--section-start::script--><script src="{{path}}"></script><!--section-end::script-->

<!--section-start::full--><!DOCTYPE html>
<html lang="en">
{{>header}}
  <body>
    <div class="container">
    {{>content}}
    </div>
  </body>
</html>
<!--section-end::full-->

<!--section-start::graph-->
<div id="{{plotId}}"> </div>
<script>
  var dates = {{>dates}};
  var infections = {{>infections}};
  var deaths = {{>deaths}};
  var traces = [];

  traces.push({
      x: dates,
      y: infections,
      type: 'scatter',
      name: 'Infections'
  });
  traces.push({
      x: dates,
      y: deaths,
      type: 'scatter',
      name: 'Deaths'
  });
  var layout = {
    title: {
      text: '{{title}}'
    },
    yaxis: {
      title: {
        text: 'Cases per day'
      }
    }
  };
  Plotly.newPlot('{{plotId}}', traces, layout, {
      displaylogo: false,
      modeBarButtonsToRemove: ['sendDataToCloud']
  });
</script>
<!--section-end::graph-->

<!--section-start::graphAccumulated-->
<div id="{{plotId}}"> </div>
<script>
  var dates = {{>dates}};
  var totalInfections = {{>infections}};
  var totalDeaths = {{>deaths}};
  var traces = [];

  traces.push({
      x: dates,
      y: totalInfections,
      type: 'scatter',
      name: 'Infections'
  });
  traces.push({
      x: dates,
      y: totalDeaths,
      type: 'scatter',
      name: 'Deaths'
  });
  var layout = {
    title: {
      text: '{{title}}'
    },
    yaxis: {
      title: {
        text: 'Accumulated number of cases'
      }
    }
  };
  Plotly.newPlot('{{plotId}}', traces, layout, {
      displaylogo: false,
      modeBarButtonsToRemove: ['sendDataToCloud']
  });
</script>
<!--section-end::graphAccumulated-->

<!--section-start::graphIncidence-->
<div id="{{plotId}}"> </div>
<script>
  var dates14 = {{>dates14}};
  var incidence14 = {{>incidence14}};
  var dates7 = {{>dates7}};
  var incidence7 = {{>incidence7}};
  var traces = [];

  traces.push({
      x: dates14,
      y: incidence14,
      type: 'scatter',
      name: '14-day incidence'
  });
  traces.push({
      x: dates7,
      y: incidence7,
      type: 'scatter',
      name: '7-day incidence'
  });
  var layout = {
    title: {
      text: '{{title}}'
    },
    yaxis: {
      title: {
        text: '14-day and 7-day incidences'
      }
    }
  };
  Plotly.newPlot('{{plotId}}', traces, layout, {
      displaylogo: false,
      modeBarButtonsToRemove: ['sendDataToCloud']
  });
</script>
<br />
<div style="text-align: center; font-style: italic;">The 14-day incidence is the number of infections per 100000 inhabitants over the last 14 days.<br />
The 7-day incidence is the number of infections per 100000 inhabitants over the last seven days.</div>
<!--section-end::graphIncidence-->

<!--section-start::graphIncidenceByYear-->
<div id="{{plotId}}"> </div>
<script>
  var traces = [];

{{>traces}}
  var layout = {
    title: {
      text: '{{title}}'
    },
    xaxis: {
      title: {
        text: 'Day of year'
      }
    },
    yaxis: {
      title: {
        text: '7-day incidences'
      }
    }
  };
  Plotly.newPlot('{{plotId}}', traces, layout, {
      displaylogo: false,
      modeBarButtonsToRemove: ['sendDataToCloud']
  });
</script>
<br />
<div style="text-align: center; font-style: italic;">The 7-day incidence is the number of infections per 100000 inhabitants over the last seven days.</div>
<!--section-end::graphIncidenceByYear-->

<!--section-start::graphContinent-->
<div id="{{plotId}}"> </div>
<script>
  var traces = [];

{{>traces}}
  var layout = {
    title: {
      text: '{{title}}'
    },
    yaxis: {
      title: {
        text: '14-day incidence'
      }
    }
  };
  Plotly.newPlot('{{plotId}}', traces, layout, {
      displaylogo: false,
      modeBarButtonsToRemove: ['sendDataToCloud'],
      responsive: true
  });
</script>
<br />
<div style="text-align: center; font-style: italic;">The 14-day incidence is the number of infections per 100000 inhabitants over the last 14 days.</div>
<!--section-end::graphContinent-->

<!--section-start::trace-->
  traces.push({
      x: {{>dates}},
      y: {{>incidence}},
      type: 'scatter',
      mode: 'lines',
      name: '{{name}}'
  });<!--section-end::trace-->

<!--section-start::index--><h1>Corona cases in various countries</h1>
<br />
<ul>{{>links}}</ul>
<!--section-end::index-->

<!--section-start::indexContinents--><h1>Coronavirus incidence by continent</h1>
<br />
<ul>{{>links}}</ul>
<!--section-end::indexContinents-->

<!--section-start::indexLink--><li><a href="{{url}}">{{text}}</a></li>
<!--section-end::indexLink-->
