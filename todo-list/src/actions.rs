use crate::{
    constants,
    types::{Json, ListType, Todo},
};

pub struct Action {
    action_name: &'static str,
    pub action_on_list: ListType,
    pub state_before_action: Todo,
}

pub struct UserActions {
    pub user_actions: Vec<Action>,
}

impl UserActions {
    pub fn push(
        &mut self,
        action_name: &'static str,
        action_on_list: ListType,
        state_before_action: Todo,
    ) {
        self.user_actions.push(Action {
            action_name,
            action_on_list,
            state_before_action,
        });
    }

    // undoing something
    pub fn pop(&mut self, parsed_json: &mut Json) {
        if self.user_actions.len() == 0 {
            return;
        }

        let last_action = self.user_actions.pop().unwrap();

        match last_action.action_name {
            constants::DELETE_ITEM => parsed_json
                .insert_into_list(last_action.action_on_list, last_action.state_before_action),
            _ => {}
        }
    }
}
