import { combineReducers} from 'redux';
import { playerProfileReducer, PlayerProfile } from './playerProfile.reducer';

export interface State {
  playerProfile : PlayerProfile;
};

const reducers = combineReducers<State>({
  playerProfile: playerProfileReducer
});

export default reducers;
