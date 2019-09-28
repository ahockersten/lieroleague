import React from 'react';
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import AddPlayer from './AddPlayer'

const App: React.FC = () =>
  <Router>
    <Switch>
      <Route path="/">
        <AddPlayer />
      </Route>
    </Switch>
  </Router>

export default App;
