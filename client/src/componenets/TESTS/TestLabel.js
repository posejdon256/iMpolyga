import React from 'react'
import { connect } from 'react-redux'
import { Text } from "react-native";

const TestLabel = (props) => <Text>{props.testText}</Text>

const mapStateToProps = state => ({
    testText: state.testLabel
})

export default connect(mapStateToProps)(TestLabel)