import { SagaIterator } from '@redux-saga/core';
import { takeLatest, call, put } from 'redux-saga/effects';
import { Actions, LoginAction, PlayerLoginData } from '../actions';
import { apiClient } from './index';

function loginApi(playerLoginData: PlayerLoginData): Promise<Response> {
  return apiClient.post('player/login', {
    json: playerLoginData
  });
}

function getProfileApi(): Promise<Response> {
  return apiClient.get('player/profile');
}

function* loginEffectSaga(action: LoginAction): SagaIterator {
  yield call(loginApi, action.payload);
  yield put({ type: Actions.WATCH_GET_PROFILE });
}

function* getProfileEffectSaga(): SagaIterator {
  const response = yield call(getProfileApi);
  const json = yield response.json();

  yield put({ type: Actions.UPDATE_PROFILE, payload: json });
}

export function* watchLoginSaga(): SagaIterator {
  yield takeLatest(Actions.WATCH_LOGIN, loginEffectSaga);
}

export function* watchGetProfileSaga(): SagaIterator {
  yield takeLatest(Actions.WATCH_GET_PROFILE, getProfileEffectSaga);
}
