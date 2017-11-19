import React, { Component } from 'react';
import './Headlines.css';

function getCurrentEvents() {
  var events = [{
    "title": "Title",
    "summary": "Summary",
    "articleMoods": [{
      source: "Source name",
      mood: 5
    }],
    date: "9:57PM"
  }, {
    "title": "Title",
    "summary": "Summary",
    "articleMoods": [{
      source: "Source name",
      mood: 5 
    }],
    date: "9:57PM"
  }, {
    "title": "Title",
    "summary": "Summary",
    "articleMoods": [{
      source: "Source name",
      mood: 5
    }],
    date: "9:57PM"
  }];

  return events;
}

var events = getCurrentEvents();

class Headlines extends Component {
  render() {
    return (
    	<div className="Headlines">
      <h2>Headlines</h2>
      <h3>Headline 1</h3>
        <ol>
          <li>{events[0].title}</li>
          <li>{events[0].summary}</li>
          <li>{events[0].articleMoods[0].source}</li>
          <li>{events[0].articleMoods[0].mood}</li>
          <li>{events[0].date}</li>
        </ol>
      <h3>Headline 2</h3>
        <ol>
          <li>{events[1].title}</li>
          <li>{events[1].summary}</li>
          <li>{events[1].articleMoods[0].source}</li>
          <li>{events[1].articleMoods[0].mood}</li>
          <li>{events[1].date}</li>
        </ol>

      <h3>Headline 2</h3>
        <ol>
          <li>{events[2].title}</li>
          <li>{events[2].summary}</li>
          <li>{events[2].articleMoods[0].source}</li>
          <li>{events[2].articleMoods[0].mood}</li>
          <li>{events[2].date}</li>
        </ol>
    	</div>
    );
  }
}

export default Headlines;