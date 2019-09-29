import { Actions, BaseAction } from '../actions';

type Nationality = string;
type TimeZone = string;
type Country = string;
type Locale = string;

export type PlayerProfile = {
  nickName?: string;
  email?: string;
  realName?: string;
  color?: PlayerColor;
  nationality?: Nationality;
  timeZone?: TimeZone;
  location?: Country;
  locale?: Locale;
};

export type PlayerColor = {
  r: number;
  b: number;
  g: number;
};

const defaultPlayerProfile = {
  nickName: undefined,
  email: undefined
};

export const playerProfileReducer = (
  state: PlayerProfile = defaultPlayerProfile,
  action: BaseAction
): PlayerProfile => {
  switch (action.type) {
    case Actions.UPDATE_PROFILE:
      return {
        ...action.payload
      };
    default:
      return state;
  }
};
