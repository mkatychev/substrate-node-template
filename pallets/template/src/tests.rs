use crate::mock::*;
use crate::Error;
use frame_support::{assert_err, assert_ok};

#[test]
fn test_increasing_value() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_value(Origin::signed(1), 1));
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::switch_state(Origin::signed(1), 1,));
		// Read pallet storage and assert an expected result.
		assert_ok!(TemplateModule::execute_action(Origin::signed(1)));
	});
}

#[test]
fn test_decreasing_value() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_value(Origin::signed(1), 2));
		assert_ok!(TemplateModule::switch_state(Origin::signed(1), 2,));
		assert_ok!(TemplateModule::execute_action(Origin::signed(1)));
	});
}

#[test]
fn test_idle_value() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::set_value(Origin::signed(1), 1));
		assert_err!(
			TemplateModule::switch_state(Origin::signed(1), 0),
			Error::<Test>::RedundantSwitch,
		);
		assert_ok!(TemplateModule::switch_state(Origin::signed(1), 1,));
		assert_ok!(TemplateModule::switch_state(Origin::signed(1), 0,));
		assert_ok!(TemplateModule::execute_action(Origin::signed(1)));
	});
}

#[test]
fn test_none_errors() {
	new_test_ext().execute_with(|| {
		assert_err!(
			TemplateModule::set_value(Origin::signed(1), 0),
			Error::<Test>::CannotBeZero,
		);
		assert_err!(
			TemplateModule::switch_state(Origin::signed(1), 0),
			Error::<Test>::SwitchOnNone,
		);
		assert_err!(
			TemplateModule::execute_action(Origin::signed(1)),
			Error::<Test>::ExecuteOnNone,
		);
	});
}

#[test]
fn test_some_errors() {
	new_test_ext().execute_with(|| {
		// init with valid value
		assert_ok!(TemplateModule::set_value(Origin::signed(1), 1));

		assert_err!(
			TemplateModule::set_value(Origin::signed(1), 2),
			Error::<Test>::SetOnSome,
		);
		assert_err!(
			TemplateModule::switch_state(Origin::signed(1), 0),
			Error::<Test>::RedundantSwitch,
		);
		assert_err!(
			TemplateModule::switch_state(Origin::signed(1), 2),
			Error::<Test>::CannotDecreaseToZero,
		);

		// test u32::MAX case
		assert_ok!(TemplateModule::set_value(Origin::signed(2), u32::MAX));
		assert_err!(
			TemplateModule::switch_state(Origin::signed(2), 1),
			Error::<Test>::CannotIncreasePastMax,
		);
		assert_err!(
			TemplateModule::switch_state(Origin::signed(2), 3),
			Error::<Test>::InvalidStateInt,
		);
	});
}
