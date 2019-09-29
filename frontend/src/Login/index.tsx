import Button from '@material-ui/core/Button';
import TextField from '@material-ui/core/TextField';
import React, {useState} from 'react';
import { connect } from 'react-redux';
import { bindActionCreators, Dispatch } from 'redux';
import { watchLogin } from '../store/actionCreators';
import { State } from '../reducers';
import { PlayerProfile } from '../reducers/playerProfile.reducer'

interface LoginProps {
  watchLogin: typeof watchLogin;
  playerProfile: PlayerProfile;
}

const Login: React.FC<LoginProps> = (props: LoginProps, state: State) => {
  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    props.watchLogin({
      email: emailField,
      password: passwordField
    });
  }
  const [emailField, setEmailField] = useState(props.playerProfile.email || "");
  const [passwordField, setPasswordField] = useState("");

  return <form onSubmit={onSubmit}>
    <TextField
      required
      id="email"
      type="email"
      name="email"
      autoComplete="email"
      label="Email"
      value={emailField}
      onChange={(e) => setEmailField(e.target.value)}
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
      onChange={(e) => setPasswordField(e.target.value)}
    />
    <Button variant="contained" color="primary" type="submit">
      Login
    </Button>
  </form>
}

const mapStateToProps = (state: State) => ({
  playerProfile: state.playerProfile
})

// mapping dispatch functions to the props of LoginForm component
const mapDispatchToProps = (dispatch: Dispatch) => {
  return bindActionCreators({
    watchLogin
    // add other watcher sagas to this object to map them to props
  }, dispatch);
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Login);
