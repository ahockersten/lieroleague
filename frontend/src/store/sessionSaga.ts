import { Actions, BaseAction, PlayerLoginData } from '../actions';
import { apiClient } from './index';
import { takeLatest, call, put } from 'redux-saga/effects';

function loginApi(playerLoginData: PlayerLoginData) {
  return apiClient.request({
    method: 'post',
    url: '/player/login',
    data: playerLoginData
  });
}

function getProfileApi() {
  return apiClient.request({
    method: 'get',
    url: '/player/profile',
  });
}

function* loginEffectSaga(action: BaseAction) {
  try {
    yield call(loginApi, action.payload);
    yield put({type: Actions.WATCH_GET_PROFILE});
  } catch (e) {
    // catch error on a bad axios call
    // alert using an alert library
  }
}

function* getProfileEffectSaga(action: BaseAction) {
  try {
    let { data } = yield call(getProfileApi);

    yield put({ type: Actions.UPDATE_PROFILE, payload: data });
  } catch (e) {
    // catch error on a bad axios call
    // alert using an alert library
  }
}

export function* watchLoginSaga() {
  yield takeLatest(Actions.WATCH_LOGIN, loginEffectSaga);
}

export function* watchGetProfileSaga() {
  yield takeLatest(Actions.WATCH_GET_PROFILE, getProfileEffectSaga);
}
