import Button from '@material-ui/core/Button';
import TextField from '@material-ui/core/TextField';
import React, { useState } from 'react';
import { connect } from 'react-redux';
import { bindActionCreators, Dispatch } from 'redux';
import { watchLogin } from '../actions';
import { State } from '../reducers';
import { PlayerProfile } from '../reducers/playerProfile.reducer';

type LoginProps = StateFromProps & DispatchFromProps;

const Login: React.FC<LoginProps> = (props: LoginProps) => {
  const [emailField, setEmailField] = useState(props.playerProfile.email || '');
  const [passwordField, setPasswordField] = useState('');
  const onSubmit = (e: React.FormEvent<HTMLFormElement>): void => {
    e.preventDefault();
    props.watchLogin({
      email: emailField,
      password: passwordField
    });
  };

  return (
    <form onSubmit={onSubmit}>
      <TextField
        required
        id="email"
        type="email"
        name="email"
        autoComplete="email"
        label="Email"
        value={emailField}
        onChange={(e): void => setEmailField(e.target.value)}
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
        value={passwordField}
        onChange={(e): void => setPasswordField(e.target.value)}
      />
      <Button variant="contained" color="primary" type="submit">
        Login
      </Button>
    </form>
  );
};

interface StateFromProps {
  playerProfile: PlayerProfile;
}

const mapStateToProps = (state: State): StateFromProps => ({
  playerProfile: state.playerProfile
});

interface DispatchFromProps {
  watchLogin: typeof watchLogin;
}

// mapping dispatch functions to the props of LoginForm component
const mapDispatchToProps = (dispatch: Dispatch): DispatchFromProps => {
  return bindActionCreators(
    {
      watchLogin
    },
    dispatch
  );
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Login);
