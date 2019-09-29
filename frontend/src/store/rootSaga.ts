import { all } from 'redux-saga/effects';
import {
  getProfileWatcherSaga,
  loginWatcherSaga
} from './sessionSaga';

export default function* rootSaga() {
  yield all([
    getProfileWatcherSaga(),
    loginWatcherSaga()
  ]);
}
