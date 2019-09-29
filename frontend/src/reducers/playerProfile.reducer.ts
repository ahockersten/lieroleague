import { Actions, BaseAction } from '../store/actionCreators';

export type PlayerProfile = {
  nickName?: string;
};

const defaultPlayerProfile = {
  nickName: undefined
}

export const playerProfileReducer = (state: PlayerProfile = defaultPlayerProfile, action: BaseAction) => {
  switch (action.type) {
    case Actions.UPDATE_PROFILE:
      return {
        ...state,
        playerProfile: action.payload
      }
    default:
      return state;
  }
}
