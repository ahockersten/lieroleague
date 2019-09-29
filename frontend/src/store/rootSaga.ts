import { SagaIterator } from '@redux-saga/core';
import { call, all } from 'redux-saga/effects';
import { watchGetProfileSaga, watchLoginSaga } from './sessionSaga';

export default function* rootSaga(): SagaIterator {
  yield all([call(watchGetProfileSaga), call(watchLoginSaga)]);
}
