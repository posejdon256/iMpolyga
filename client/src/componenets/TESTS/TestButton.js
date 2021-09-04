import React from "react";
import { Button } from "react-native";
import { connect } from "react-redux";
import { pingServerAction } from "../../actions/test-action";

const TestButton = (props) => (
  <Button onPress={props.onClick} title={`Send message to server`} />
);

const mapDispatchProps = (dispatch) => {
  return {
    onClick: () => dispatch(pingServerAction()),
  };
};
export default connect(null, mapDispatchProps)(TestButton);
