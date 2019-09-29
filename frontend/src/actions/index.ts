export enum Actions {
  WATCH_LOGIN = 'WATCH_LOGIN',
  WATCH_GET_PROFILE = 'WATCH_GET_PROFILE',
  UPDATE_PROFILE = 'UPDATE_PROFILE'
}

export interface BaseAction {
  type: Actions;
  payload: object;
}

export interface LoginAction {
  type: Actions;
  payload: PlayerLoginData;
}

export interface PlayerLoginData {
  email: string;
  password: string;
}

export function watchLogin(playerLoginData: PlayerLoginData): BaseAction {
  return { type: Actions.WATCH_LOGIN, payload: playerLoginData };
}

export function watchGetProfile(): BaseAction {
  return { type: Actions.WATCH_GET_PROFILE, payload: {} };
}

// FIXME add more
export interface PlayerProfileData {
  nickName: string;
}

export function updateProfile(
  playerProfileData: PlayerProfileData
): BaseAction {
  return { type: Actions.UPDATE_PROFILE, payload: playerProfileData };
}
