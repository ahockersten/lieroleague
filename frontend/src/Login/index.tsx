import Button from '@material-ui/core/Button';
import TextField from '@material-ui/core/TextField';
import React from 'react';
import { connect } from 'react-redux';
import { bindActionCreators, Dispatch } from 'redux';
import { loginWatcher } from '../store/actionCreators';
import { State } from '../reducers';

interface LoginProps {
  loginWatcher: typeof loginWatcher;
}

const Login: React.FC<LoginProps> = (props: LoginProps, state: State) => {
  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    props.loginWatcher({
      email: "test@example.com",
      password: "pass"
    });
  }

  return <form onSubmit={onSubmit}>
    <TextField
      required
      id="email"
      type="email"
      name="email"
      autoComplete="email"
      label="Email"
      defaultValue=""
      margin="normal"
      variant="outlined"
    />
    <TextField
      id="password-input"
      label="Password"
      type="password"
      autoComplete="new-password"
      margin="normal"
      variant="outlined"
    />
    <Button variant="contained" color="primary" type="submit">
      Login
    </Button>
  </form>
}

const mapStateToProps = (state: State) => ({
})

// mapping dispatch functions to the props of LoginForm component
const mapDispatchToProps = (dispatch: Dispatch) => {
  return bindActionCreators({
    loginWatcher
    // add other watcher sagas to this object to map them to props
  }, dispatch);
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Login);
