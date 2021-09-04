import { combineReducers, createStore } from "redux";
import testLabel from "../reducers/test-reducer";
import mapObject from "../reducers/map";

export default createStore(combineReducers({ testLabel, mapObject }), {});
