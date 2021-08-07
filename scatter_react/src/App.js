import React from 'react';
import {BrowserRouter as Router, Switch, Route} from 'react-router-dom';
import Graph from './components/Graph';

function App() {
  return (
    <>
      <Router>
        <Switch>
          <Route path="/" exact component={Graph}/>
        </Switch>
      </Router>
    </>
  );
}

export default App;
