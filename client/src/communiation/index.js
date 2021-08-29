import { testAction } from "../actions/test-action.js"
import store from "../store/index.js"

export const pingServer = () => {
    //Here you can perform action which will ping server

    setTimeout(()=>{
        store.dispatch(testAction("lol, why not"));
    },1000);
}
export const updateSomeData = (stringData) => {
    //This will update test label
    store.dispatch(testAction(stringData))
}
