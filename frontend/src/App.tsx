import React from 'react';
import { Provider } from 'react-redux';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import AddPlayer from './AddPlayer';
import Login from './Login';
import store from './store';

const App: React.FC = () => (
  <Provider store={store}>
    <Router>
      <Switch>
        <Route path="/add-player">
          <AddPlayer />
        </Route>
        <Route path="/">
          <Login />
        </Route>
      </Switch>
    </Router>
  </Provider>
);

export default App;
