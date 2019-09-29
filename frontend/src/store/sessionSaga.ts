import axios from 'axios';
import { Actions, BaseAction, updateProfile, PlayerLoginData } from './actionCreators';
import { takeLatest, call, put } from 'redux-saga/effects';

const apiClient = axios.create({
  baseURL: 'http://localhost:8000',
  withCredentials: true
})

function loginApi(playerLoginData: PlayerLoginData) {
  return apiClient.request({
    method: 'post',
    url: '/player/login',
    data: playerLoginData
  });
}

/** function that returns an axios call */
function getProfileApi() {
  return apiClient.request({
    method: 'get',
    url: '/player/profile',
  });
}

/** saga worker that is responsible for the side effects */
function* loginEffectSaga(action: BaseAction) {
  try {
    yield call(loginApi, action.payload);
    yield put({ type: Actions.GET_PROFILE_WATCHER });
  } catch (e) {
    // catch error on a bad axios call
    // alert using an alert library
  }
}

function * getProfileEffectSaga(action: BaseAction) {
  try {
    let { data } = yield call(getProfileApi);

    // dispatch action to change redux state
    yield put(updateProfile(data.profile));
  } catch (e) {
    // catch error on a bad axios call
    // alert using an alert library
  }
}

export function* loginWatcherSaga() {
  yield takeLatest(Actions.LOGIN_WATCHER, loginEffectSaga);
}

export function* getProfileWatcherSaga() {
  yield takeLatest(Actions.GET_PROFILE_WATCHER, getProfileEffectSaga);
}
