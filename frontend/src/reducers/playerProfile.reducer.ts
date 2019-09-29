import { Actions, BaseAction } from '../actions';

export type PlayerProfile = {
  nickName?: string;
  email?: string;
};

const defaultPlayerProfile = {
  nickName: undefined,
  email: undefined
}

export const playerProfileReducer = (state: PlayerProfile = defaultPlayerProfile, action: BaseAction) => {
  switch (action.type) {
    case Actions.UPDATE_PROFILE:
      return {
        ...action.payload
      }
    default:
      return state;
  }
}
