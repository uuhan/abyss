import { configureStore } from '@reduxjs/toolkit'
import counterReducer from './features/counter'

export default configureStore({
  reducer: {
    counter: counterReducer,
  }
})
