import React from 'react';
import { Provider } from 'react-redux';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import AddPlayer from './AddPlayer';
import Header from './Header';
import Login from './Login';
import store from './store';
import CssBaseline from '@material-ui/core/CssBaseline';

import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    body: {
      maxWidth: 1024,
      width: '90%',
      margin: 'auto',
      marginTop: 16,
      marginBottom: 16,
    },
  }),
);


const App: React.FC = () => {

  const classes = useStyles();

  return (
    <Provider store={store}>
      <CssBaseline />
      <Header></Header>
      <div className={classes.body}>
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
      </div>
    </Provider>
  );

};

export default App;
