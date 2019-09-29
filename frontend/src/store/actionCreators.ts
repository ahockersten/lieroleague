export enum Actions {
  LOGIN_WATCHER = 'LOGIN_WATCHER',
  GET_PROFILE_WATCHER = 'GET_PROFILE_WATCHER',
  UPDATE_PROFILE = 'UPDATE_PROFILE',
}

export interface BaseAction {
  type: Actions;
  payload: any;
}

export interface PlayerLoginData {
  email: string;
  password: string;
}

export function loginWatcher(playerLoginData: PlayerLoginData): BaseAction {
  return { type: Actions.LOGIN_WATCHER, payload: playerLoginData };
}

export function getProfileWatcher(): BaseAction {
  return { type: Actions.GET_PROFILE_WATCHER, payload: {} };
}

// FIXME add more
export interface PlayerProfileData {
  nickName: string;
}

export function updateProfile(playerProfileData: PlayerProfileData): BaseAction {
  return { type: Actions.UPDATE_PROFILE, payload: playerProfileData };
}
