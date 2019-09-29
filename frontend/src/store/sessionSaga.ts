import { SagaIterator } from '@redux-saga/core';
import { takeLatest, call, put } from 'redux-saga/effects';
import { Actions, LoginAction, PlayerLoginData } from '../actions';
import { apiClient } from './index';
import { PlayerProfile } from '../reducers/playerProfile.reducer';

function loginApi(playerLoginData: PlayerLoginData): Promise<void> {
  return apiClient.request({
    method: 'post',
    url: '/player/login',
    data: playerLoginData
  });
}

function getProfileApi(): Promise<PlayerProfile> {
  return apiClient.request({
    method: 'get',
    url: '/player/profile'
  });
}

function* loginEffectSaga(action: LoginAction): SagaIterator {
  try {
    yield call(loginApi, action.payload);
    yield put({ type: Actions.WATCH_GET_PROFILE });
  } catch (e) {
    // catch error on a bad axios call
    // alert using an alert library
  }
}

function* getProfileEffectSaga(): SagaIterator {
  try {
    const { data } = yield call(getProfileApi);

    yield put({ type: Actions.UPDATE_PROFILE, payload: data });
  } catch (e) {
    // catch error on a bad axios call
    // alert using an alert library
  }
}

export function* watchLoginSaga(): SagaIterator {
  yield takeLatest(Actions.WATCH_LOGIN, loginEffectSaga);
}

export function* watchGetProfileSaga(): SagaIterator {
  yield takeLatest(Actions.WATCH_GET_PROFILE, getProfileEffectSaga);
}
