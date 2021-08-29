import { combineReducers, createStore } from "redux";
import testLabel from "../reducers/test-reducer";


export default createStore(combineReducers({testLabel}), {})