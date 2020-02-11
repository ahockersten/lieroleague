import React from 'react';
// import { Provider } from 'react-redux';
// import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';

import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      flexGrow: 1
    },
    menuButton: {
      marginRight: theme.spacing(2)
    },
    title: {
      flexGrow: 1
    },
    toolBar: {
      backgroundColor: '#333333',
      minHeight: 40,
      fontFamily: 'Verdana',
      fontSize: 10
    },
    button: {
      // fontSize: 10, doesn't apply
      borderRadius: 0,
      marginLeft: 8,
      marginRight: 8,
      '&:hover': {
        color: '#bbb',
        backgroundColor: '#282828'
      }
    }
  })
);

const Header: React.FC = () => {
  const classes = useStyles();

  return (
    <div className={classes.root}>
      <AppBar position="static">
        <Toolbar className={classes.toolBar}>
          <IconButton
            edge="start"
            className={classes.menuButton}
            color="inherit"
            aria-label="menu"
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" className={classes.title}>
            LieroLeague
          </Typography>
          <Button color="inherit" className={classes.button}>
            Players
          </Button>
          <Button color="inherit" className={classes.button}>
            Maps
          </Button>
          <Button color="inherit" className={classes.button}>
            Profile
          </Button>
          <Button color="inherit" className={classes.button}>
            Login
          </Button>
        </Toolbar>
      </AppBar>
    </div>
  );
};

export default Header;
