import Button from '@material-ui/core/Button';
import MenuItem from '@material-ui/core/MenuItem';
import Select from '@material-ui/core/Select';
import Slider from '@material-ui/core/Slider';
import TextField from '@material-ui/core/TextField';
import Grid from '@material-ui/core/Grid';
import Typography from '@material-ui/core/Typography';
import React from 'react';
import './add-player.css';

import { createStyles, makeStyles /*, Theme*/ } from '@material-ui/core/styles';

const useStyles = makeStyles((/*theme: Theme*/) =>
  createStyles({
    addplayerform: {
      width: '60%',
      margin: 'auto',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      alignItems: 'center'
    },
    textfield: {
      width: '70%',
      paddingTop: 4,
      paddingBottom: 4,
      paddingLeft: 4,
      paddingRight: 4,
      margin: 4,
      fontSize: 10,
      boxSizing: 'inherit'
    },
    select: {
      minWidth: 200
    },
    grid: {
      margin: 4,
      width: '65%'
    }
  }));

const AddPlayer: React.FC = () => {
  const classes = useStyles();

  return (
    <form className={classes.addplayerform}>
      <TextField
        className={classes.textfield}
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
        className={classes.textfield}
        required
        id="password-input"
        label="Password"
        type="password"
        autoComplete="new-password"
        margin="normal"
        variant="outlined"
      />
      <TextField
        required
        className={classes.textfield}
        id="repeat-password-input"
        label="Repeat password"
        type="password"
        autoComplete="new-password"
        margin="normal"
        variant="outlined"
      />
      <TextField
        className={classes.textfield}
        required
        id="nickname"
        label="Nickname"
        autoComplete="nickname"
        defaultValue=""
        margin="normal"
        variant="outlined"
      />
      <TextField
        className={classes.textfield}
        id="real-name"
        label="Real name"
        autoComplete="name"
        defaultValue=""
        margin="normal"
        variant="outlined"
      />
      <ColorPicker />

      <Grid
        container
        className={classes.grid}
        direction="row"
        justify="space-between"
        alignItems="center"
      >
        <Typography>Nationality: </Typography>
        <Select
          className={classes.select}
          value={'Swedish'}
          inputProps={{
            name: 'nationality',
            id: 'nationality'
          }}
        >
          <MenuItem value={'Polish'}>Polish</MenuItem>
          <MenuItem value={'Swedish'}>Swedish</MenuItem>
        </Select>
      </Grid>

      <Grid
        container
        className={classes.grid}
        direction="row"
        justify="space-between"
        alignItems="center"
      >
        <Typography>Timezone: </Typography>
        <Select
          className={classes.select}
          value={'CEST'}
          inputProps={{
            name: 'timezone',
            id: 'timezone'
          }}
        >
          <MenuItem value={'CEST'}>CEST</MenuItem>
          <MenuItem value={'GMT'}>GMT</MenuItem>
        </Select>
      </Grid>

      <Grid
        container
        className={classes.grid}
        direction="row"
        justify="space-between"
        alignItems="center"
      >
        <Typography>Location: </Typography>
        <Select
          className={classes.select}
          value={'Sweden'}
          inputProps={{
            name: 'location',
            id: 'location'
          }}
        >
          <MenuItem value={'Poland'}>Poland</MenuItem>
          <MenuItem value={'Sweden'}>Sweden</MenuItem>
        </Select>
      </Grid>
      <Button variant="contained" color="primary" type="submit">
        Add player
      </Button>
    </form>
  );
};
const ColorPicker: React.FC = () => (
  <div>
    <Typography id="red-picker" gutterBottom>
      Red
    </Typography>
    <Slider
      defaultValue={0}
      aria-labelledby="red-picker"
      valueLabelDisplay="auto"
      step={1}
      min={0}
      max={255}
    />
    <Typography id="green-picker" gutterBottom>
      Green
    </Typography>
    <Slider
      defaultValue={0}
      aria-labelledby="green-picker"
      valueLabelDisplay="auto"
      step={1}
      min={0}
      max={255}
    />
    <Typography id="blue-picker" gutterBottom>
      Blue
    </Typography>
    <Slider
      defaultValue={0}
      aria-labelledby="blue-picker"
      valueLabelDisplay="auto"
      step={1}
      min={0}
      max={255}
    />
  </div>
);

export default AddPlayer;
