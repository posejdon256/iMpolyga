export const testAction = (stringValue) => {
    return {
        type: 'CHANGE_LABEL',
        payload: {
            label: stringValue
        }
    }
}
export const pingServerAction = () => {
    return {
        type: 'PING_SERVER'
    }
}
