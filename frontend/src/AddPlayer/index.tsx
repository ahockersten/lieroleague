import Button from '@material-ui/core/Button';
import FormControl from '@material-ui/core/FormControl';
import MenuItem from '@material-ui/core/MenuItem';
import Select from '@material-ui/core/Select';
import Slider from '@material-ui/core/Slider';
import TextField from '@material-ui/core/TextField';
import Typography from '@material-ui/core/Typography';
import React from 'react';

const AddPlayer: React.FC = () =>
  <FormControl>
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
    <TextField
      id="repeat-password-input"
      label="Repeat password"
      type="password"
      autoComplete="new-password"
      margin="normal"
      variant="outlined"
    />
    <TextField
      required
      id="nickname"
      label="Nickname"
      autoComplete="nickname"
      defaultValue=""
      margin="normal"
      variant="outlined"
    />
    <TextField
      id="real-name"
      label="Real name"
      autoComplete="name"
      defaultValue=""
      margin="normal"
      variant="outlined"
    />
    <ColorPicker />
    <Select
      value={"Swedish"}
      onChange={() => {}}
      inputProps={{
        name: 'nationality',
        id: 'nationality',
      }}
    >
      <MenuItem value={"Polish"}>Polish</MenuItem>
      <MenuItem value={"Swedish"}>Swedish</MenuItem>
    </Select>
    <Select
      value={"CEST"}
      onChange={() => {}}
      inputProps={{
        name: 'timezone',
        id: 'timezone',
      }}
    >
      <MenuItem value={"CEST"}>CEST</MenuItem>
      <MenuItem value={"GMT"}>GMT</MenuItem>
    </Select>
    <Select
      value={"Sweden"}
      onChange={() => {}}
      inputProps={{
        name: 'location',
        id: 'location',
      }}
    >
      <MenuItem value={"Poland"}>Poland</MenuItem>
      <MenuItem value={"Sweden"}>Sweden</MenuItem>
    </Select>
    <Button variant="contained" color="primary">
      Add player
    </Button>
  </FormControl>

const ColorPicker: React.FC = () =>
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

export default AddPlayer;
