import { all } from 'redux-saga/effects';
import {
  watchGetProfileSaga,
  watchLoginSaga
} from './sessionSaga';

export default function* rootSaga() {
  yield all([
    watchGetProfileSaga(),
    watchLoginSaga()
  ]);
}
