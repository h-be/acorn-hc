use hdk3::prelude::*;


/*
  ENTRY TYPES
*/

#[hdk_entry(id = "project_meta")]
#[derive(Debug, SerializedBytes, Clone, PartialEq)]
pub struct ProjectMeta {
  creator_address: Address,
  created_at: u128,
  name: String,
  image: Option<String>,
  passphrase: String,
}

// The "Entry" in EntryPoint is not a reference to Holochain "Entries"
// it is rather the concept of an Entrance, as in a doorway, to the tree
#[hdk_entry(id = "entry_point")]
#[derive(Debug, SerializedBytes, Clone, PartialEq)]
pub struct EntryPoint {
  color: String,
  creator_address: Address,
  created_at: u128,
  goal_address: Address,
}

// This is a reference to the agent address for any users who have joined this DHT
#[hdk_entry(id = "member")]
#[derive(Debug, SerializedBytes, Clone, PartialEq)]
pub struct Member {
  pub address: String,
}

// a relationship between a Goal and an Agent
#[hdk_entry(id = "goal_member")]
#[derive(Debug, SerializedBytes, Clone, PartialEq)]
pub struct GoalMember {
  goal_address: Address,
  agent_address: Address,
  user_edit_hash: Option<Address>,
  unix_timestamp: u128,
}

#[hdk_entry(id = "goal_vote")]
#[derive(Debug, SerializedBytes, Clone, PartialEq)]
pub struct GoalVote {
  goal_address: Address,
  urgency: f32,
  importance: f32,
  impact: f32,
  effort: f32,
  agent_address: Address,
  unix_timestamp: u128,
}


#[hdk_entry(id = "goal_comment")]
#[derive(Debug, SerializedBytes, Clone, PartialEq)]
pub struct GoalComment {
  goal_address: Address,
  content: String,
  agent_address: Address,
  unix_timestamp: u128,
}

// An edge. This is an arrow on the SoA Tree which directionally links
// two goals.
#[hdk_entry(id = "edge")]
#[derive(Debug, SerializedBytes, Clone, PartialEq)]
pub struct Edge {
  parent_address: Address,
  child_address: Address,
  randomizer: u128,
}


// A Goal Card. This is a card on the SoA Tree which can be small or non-small, complete or
// incomplete, certain or uncertain, and contains text content.
// user hash and unix timestamp are included to prevent hash collisions.
#[hdk_entry(id = "goal")]
#[derive(Debug, SerializedBytes, Clone, PartialEq)]
pub struct Goal {
  content: String,
  user_hash: Address,
  user_edit_hash: Option<Address>,
  timestamp_created: u128,
  timestamp_updated: Option<u128>,
  hierarchy: Hierarchy,
  status: Status,
  tags: Option<Vec<String>>,
  description: String,
  time_frame: Option<TimeFrame>,
}

/*

  END ENTRY TYPES

*/


#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct GoalEdgeInput {
  parent_address: Address,
  randomizer: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
pub struct GetResponse<T> {
  pub entry: T,
  pub address: Address,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct EntryPointResponse {
  entry_point: EntryPoint,
  goal: Goal,
  entry_point_address: Address,
}


/* 
  substructures of a Goal
*/
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub enum Status {
  Uncertain,
  Incomplete,
  InProcess,
  Complete,
  InReview,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub enum Hierarchy {
  Root,
  Trunk,
  Branch,
  Leaf,
  NoHierarchy,
}
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct TimeFrame {
  from_date: u128,
  to_date: u128,
}
/*

  end substructures of a Goal

  */

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct GoalMaybeWithEdge {
  goal: GetResponse<Goal>,
  maybe_edge: Option<GetResponse<Edge>>,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct ArchiveGoalResponse {
  address: Address,
  archived_edges: Vec<Address>,
  archived_goal_members: Vec<Address>,
  archived_goal_votes: Vec<Address>,
  archived_goal_comments: Vec<Address>,
  archived_entry_points: Vec<Address>,
}
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct GetHistoryResponse {
  entries: Vec<Goal>,
  members: Vec<Vec<GoalMember>>,
  address: Address,
}


pub fn init() -> Result<(), String> {
  let members_anchor_entry = Entry::App(
    "anchor".into(), // app entry type
    // app entry value. We'll use the value to specify what this anchor is for
    "members".into(),
  );
  let member = Member {
    address: AGENT_ADDRESS.to_string(),
  };
  let member_entry = Entry::App("member".into(), member.clone().into());
  let member_address = hdk::commit_entry(&member_entry)?;
  hdk::link_entries(
    &members_anchor_entry.address(),
    &member_address,
    "anchor->member",
    "",
  )?;

  // send update to peers
  // notify_member(member.clone())?;
  Ok(())
}



pub fn create_project_meta(projectmeta: ProjectMeta) -> ExternResult<GetResponse<ProjectMeta>> {
  let projectmeta_anchor_entry = Entry::App(
    "anchor".into(), // app entry type
    // app entry value. We'll use the value to specify what this anchor is for
    "projectmeta".into(),
  );
  let projectmeta_entry = Entry::App("projectmeta".into(), projectmeta.clone().into());
  let projectmeta_address = hdk::commit_entry(&projectmeta_entry)?;
  hdk::link_entries(
    &projectmeta_anchor_entry.address(),
    &projectmeta_address,
    "anchor->projectmeta",
    "",
  )?;

  Ok(GetResponse {
    entry: projectmeta,
    address: projectmeta_address,
  })
}

pub fn update_project_meta(
  projectmeta: ProjectMeta,
  address: Address,
) -> ExternResult<GetResponse<ProjectMeta>> {
  let projectmeta_entry = Entry::App("projectmeta".into(), projectmeta.clone().into());
  hdk::update_entry(projectmeta_entry, &address)?;
  Ok(GetResponse {
    entry: projectmeta,
    address: address,
  })
}

pub fn fetch_project_meta() -> ExternResult<GetResponse<ProjectMeta>> {
  let projectmeta_anchor_entry = Entry::App("anchor".into(), "projectmeta".into());
  match hdk::utils::get_links_and_load_type::<ProjectMeta>(
    &projectmeta_anchor_entry.address(),
    LinkMatch::Exactly("anchor->projectmeta"), // the link type to match
    LinkMatch::Any,
  )?
  .first()
  {
    Some(projectmeta) => {
      let app_entry = Entry::App("projectmeta".into(), projectmeta.into());
      Ok(GetResponse {
        entry: projectmeta.clone(),
        address: app_entry.address(),
      })
    }
    None => Err(ZomeApiError::Internal("no project meta exists".into())),
  }
}

pub fn create_entry_point(entry_point: EntryPoint) -> ExternResult<EntryPointResponse> {
  // if the goal doesn't exist, this will return an error for this function
  // which is right
  let goal = hdk::utils::get_as_type::<Goal>(entry_point.goal_address.clone())?;

  let app_entry = Entry::App("entry_point".into(), entry_point.clone().into());
  let entry_address = hdk::commit_entry(&app_entry)?;

  // link new entry_point to the entry_points anchor
  let anchor_address = Entry::App(
    "anchor".into(),       // app entry type
    "entry_points".into(), // app entry value
  )
  .address();

  hdk::link_entries(
    &anchor_address,
    &app_entry.address(),
    "anchor->entry_point",
    "",
  )?;

  let entry_point_response = EntryPointResponse {
    entry_point,
    goal,
    entry_point_address: entry_address,
  };
  // notify_entry_point(entry_point_response.clone())?;

  Ok(entry_point_response)
}

pub fn archive_entry_point(address: Address) -> ExternResult<Address> {
  hdk::remove_entry(&address)?;
  // notify_entry_point_archived(address.clone())?;
  Ok(address)
}

pub fn fetch_entry_points() -> ExternResult<Vec<EntryPointResponse>> {
  inner_fetch_entry_points()
}

pub fn history_of_goal(address: Address) -> ExternResult<GetHistoryResponse> {
  let anchor_address = Entry::App(
    "anchor".into(),       // app entry type
    "goal_members".into(), // app entry value
  )
  .address();
  // return all the Goal objects from the entries linked to the edge anchor (drop entries with wrong type)
  let members = hdk::get_links(
    &anchor_address,
    LinkMatch::Exactly("anchor->goal_member"), // the link type to match
    LinkMatch::Any,
  )?
  // scoop all these entries up into an array and return it
  .addresses()
  .into_iter()
  .map(|member_address: Address| {
    if let Ok(Some(entry_history)) = hdk::api::get_entry_history(&member_address) {
      Some(
        entry_history
          .items
          .into_iter()
          .map(|item| {
            if let Some(App(_, value_entry)) = item.entry {
              match serde_json::from_str::<GoalMember>(&Into::<String>::into(value_entry)).ok() {
                Some(goal_member) => {
                  // filter down to only Goal Members that are associated with the requested Goal
                  if goal_member.goal_address == address {
                    Ok(goal_member)
                  } else {
                    Err(ZomeApiError::Internal("error".into()))
                  }
                }
                None => Err(ZomeApiError::Internal("error".into())),
              }
            } else {
              Err(ZomeApiError::Internal("error".into()))
            }
          })
          .filter_map(Result::ok)
          .collect(),
      )
    } else {
      None
    }
  })
  .filter_map(|op: Option<Vec<GoalMember>>| match op {
    Some(vec) => {
      if vec.len() > 0 {
        Some(vec)
      } else {
        None
      }
    }
    _ => None,
  })
  .collect();
  if let Ok(Some(entry_history)) = hdk::api::get_entry_history(&address) {
    Ok(GetHistoryResponse {
      entries: entry_history
        .items
        .into_iter()
        .map(|item| {
          if let Some(App(_, value_entry)) = item.entry {
            match serde_json::from_str::<Goal>(&Into::<String>::into(value_entry)).ok() {
              Some(goal) => Ok(goal),
              None => Err(ZomeApiError::Internal("error".into())),
            }
          } else {
            Err(ZomeApiError::Internal("error".into()))
          }
        })
        .filter_map(Result::ok)
        .collect(),
      members: members,
      address: address,
    })
  } else {
    Err(ZomeApiError::Internal("error".into()))
  }
}

pub fn create_goal(
  goal: Goal,
  maybe_goal_edge_input: Option<GoalEdgeInput>,
) -> ExternResult<GoalMaybeWithEdge> {
  let app_entry = Entry::App("goal".into(), goal.clone().into());
  let entry_address = hdk::commit_entry(&app_entry)?;

  // link new goal to the goals anchor
  let anchor_address = Entry::App(
    "anchor".into(), // app entry type
    "goals".into(),  // app entry value
  )
  .address();

  hdk::link_entries(&anchor_address, &app_entry.address(), "anchor->goal", "")?;

  // if a parent address was provided, link the goal with its parent
  let maybe_edge = match maybe_goal_edge_input {
    Some(goal_edge_input) => {
      let edge: Edge = Edge {
        parent_address: goal_edge_input.parent_address,
        child_address: entry_address.clone(),
        randomizer: goal_edge_input.randomizer,
      };
      let edge_address = inner_create_edge(&edge)?;
      Some(GetResponse {
        entry: edge,
        address: edge_address,
      })
    }
    None => None,
  };

  let goal_maybe_with_edge = GoalMaybeWithEdge {
    goal: GetResponse {
      entry: goal,
      address: entry_address,
    },
    maybe_edge,
  };
  // notify_goal_maybe_with_edge(goal_maybe_with_edge.clone())?;
  // format the response as a GetResponse
  Ok(goal_maybe_with_edge)
}

fn inner_create_edge(edge: &Edge) -> ExternResult<Address> {
  let app_entry = Entry::App("edge".into(), edge.clone().into());
  let entry_address = hdk::commit_entry(&app_entry)?;

  // link new edge to the edges anchor
  let anchor_address = Entry::App(
    "anchor".into(), // app entry type
    "edges".into(),  // app entry value
  )
  .address();

  hdk::link_entries(&anchor_address, &app_entry.address(), "anchor->edge", "")?;

  Ok(entry_address)
}

pub fn update_goal(goal: Goal, address: Address) -> ExternResult<GetResponse<Goal>> {
  let mut goal = goal;
  let old_goal = hdk::utils::get_as_type::<Goal>(address.clone())?;
  if goal.timestamp_updated == None {
    goal.timestamp_updated = Some(goal.timestamp_created);
  }
  goal.timestamp_created = old_goal.timestamp_created;
  goal.user_edit_hash = Some(AGENT_ADDRESS.clone());
  let app_entry = Entry::App("goal".into(), goal.clone().into());
  let _ = hdk::update_entry(app_entry, &address)?;

  let goal = GetResponse {
    entry: goal,
    address,
  };
  // be careful of this GOTCHA...
  // notify_goal_maybe_with_edge
  // returns a different type response than this function
  // instead of the normal case where the types match
  // let goal_maybe_with_edge = GoalMaybeWithEdge {
  //   goal: goal.clone(),
  //   maybe_edge: None,
  // };
  // notify_goal_maybe_with_edge(goal_maybe_with_edge)?;

  // format the response as a GetResponse
  // pass the OLD address back and allow the UI to continue to use it
  Ok(goal)
}

pub fn create_edge(edge: Edge) -> ExternResult<GetResponse<Edge>> {
  let entry_address = inner_create_edge(&edge)?;

  let get_response = GetResponse {
    entry: edge,
    address: entry_address,
  };

  // notify_edge(get_response.clone())?;

  Ok(get_response)
}

pub fn update_edge(edge: Edge, address: Address) -> ExternResult<GetResponse<Edge>> {
  let app_entry = Entry::App("edge".into(), edge.clone().into());
  let _ = hdk::update_entry(app_entry, &address)?;

  let get_response = GetResponse {
    entry: edge,
    address: address,
  };

  // notify_edge(get_response.clone())?;

  Ok(get_response)
}

pub fn fetch_goals() -> ExternResult<Vec<GetResponse<Goal>>> {
  // set up the anchor entry and compute its address
  let anchor_address = Entry::App(
    "anchor".into(), // app entry type
    "goals".into(),  // app entry value
  )
  .address();

  Ok(
    // return all the Goal objects from the entries linked to the edge anchor (drop entries with wrong type)
    hdk::get_links(
      &anchor_address,
      LinkMatch::Exactly("anchor->goal"), // the link type to match
      LinkMatch::Any,
    )?
    // scoop all these entries up into an array and return it
    .addresses()
    .into_iter()
    .map(
      |address: Address| match hdk::utils::get_as_type(address.clone()) {
        Ok(goal) => Ok(GetResponse {
          entry: goal,
          address,
        }),

        Err(e) => Err(e),
      },
    )
    .filter_map(Result::ok)
    .collect(),
  )
}

fn inner_fetch_goal_members() -> ExternResult<Vec<GetResponse<GoalMember>>> {
  // set up the anchor entry and compute its address
  let anchor_address = Entry::App(
    "anchor".into(),       // app entry type
    "goal_members".into(), // app entry value
  )
  .address();

  Ok(
    // return all the GoalMember objects from the entries linked to the goal_members anchor (drop entries with wrong type)
    hdk::utils::get_links_and_load_type(
      &anchor_address,
      LinkMatch::Exactly("anchor->goal_member"), // the link type to match
      LinkMatch::Any,
    )?
    .into_iter()
    .map(|goal_member: GoalMember| {
      // re-create the goal_member entry to find its address
      let address = Entry::App("goal_member".into(), goal_member.clone().into()).address();
      // return a response structs with the edge and its address
      GetResponse {
        entry: goal_member,
        address,
      }
    })
    .collect(),
  )
}
fn inner_fetch_goal_votes() -> ExternResult<Vec<GetResponse<GoalVote>>> {
  // set up the anchor entry and compute its address
  let anchor_address = Entry::App(
    "anchor".into(),     // app entry type
    "goal_votes".into(), // app entry value
  )
  .address();

  Ok(
    // return all the GoalMember objects from the entries linked to the goal_members anchor (drop entries with wrong type)
    hdk::utils::get_links_and_load_type(
      &anchor_address,
      LinkMatch::Exactly("anchor->goal_vote"), // the link type to match
      LinkMatch::Any,
    )?
    .into_iter()
    .map(|goal_vote: GoalVote| {
      // re-create the goal_member entry to find its address
      let address = Entry::App("goal_vote".into(), goal_vote.clone().into()).address();
      // return a response structs with the edge and its address
      GetResponse {
        entry: goal_vote,
        address,
      }
    })
    .collect(),
  )
}
fn inner_fetch_goal_comments() -> ExternResult<Vec<GetResponse<GoalComment>>> {
  // set up the anchor entry and compute its address
  let anchor_address = Entry::App(
    "anchor".into(),        // app entry type
    "goal_comments".into(), // app entry value
  )
  .address();

  Ok(
    // return all the GoalMember objects from the entries linked to the goal_members anchor (drop entries with wrong type)
    hdk::utils::get_links_and_load_type(
      &anchor_address,
      LinkMatch::Exactly("anchor->goal_comment"), // the link type to match
      LinkMatch::Any,
    )?
    .into_iter()
    .map(|goal_comment: GoalComment| {
      // re-create the goal_member entry to find its address
      let address = Entry::App("goal_comment".into(), goal_comment.clone().into()).address();
      // return a response structs with the edge and its address
      GetResponse {
        entry: goal_comment,
        address,
      }
    })
    .collect(),
  )
}
fn inner_fetch_edges() -> ExternResult<Vec<GetResponse<Edge>>> {
  // set up the anchor entry and compute its address
  let anchor_address = Entry::App(
    "anchor".into(), // app entry type
    "edges".into(),  // app entry value
  )
  .address();

  Ok(
    hdk::get_links(
      &anchor_address,
      LinkMatch::Exactly("anchor->edge"), // the link type to match
      LinkMatch::Any,
    )?
    // scoop all these entries up into an array and return it
    .addresses()
    .into_iter()
    .map(
      |address: Address| match hdk::utils::get_as_type(address.clone()) {
        Ok(edge) => Ok(GetResponse {
          entry: edge,
          address,
        }),

        Err(e) => Err(e),
      },
    )
    .filter_map(Result::ok)
    .collect(),
  )
}
pub fn inner_fetch_entry_points() -> ExternResult<Vec<EntryPointResponse>> {
  let anchor_address = Entry::App(
    "anchor".into(), // app entry type
    // app entry value. We'll use the value to specify what this anchor is for
    "entry_points".into(),
  )
  .address();

  let entry_points = hdk::utils::get_links_and_load_type(
    &anchor_address,
    LinkMatch::Exactly("anchor->entry_point"), // the link type to match
    LinkMatch::Any,
  )?
  .into_iter()
  .map(|entry_point: EntryPoint| {
    let entry_address = Entry::App("entry_point".into(), entry_point.clone().into()).address();
    match hdk::utils::get_as_type(entry_point.goal_address.clone()) {
      Ok(goal) => Ok(EntryPointResponse {
        entry_point: entry_point,
        goal: goal,
        entry_point_address: entry_address,
      }),
      Err(e) => Err(e),
    }
  })
  .filter_map(Result::ok)
  .collect();

  Ok(entry_points)
}

pub fn fetch_edges() -> ExternResult<Vec<GetResponse<Edge>>> {
  inner_fetch_edges()
}

pub fn archive_goal(address: Address) -> ExternResult<ArchiveGoalResponse> {
  // commit the removeEntry. Returns the address of the removeEntry
  hdk::remove_entry(&address)?;

  let archived_edges = inner_fetch_edges()?
    .into_iter()
    .filter(|get_response: &GetResponse<Edge>| {
      // check whether the parent_address or child_address is equal to the given address.
      // If so, the edge is connected to the goal being archived.
      get_response.entry.child_address == address || get_response.entry.parent_address == address
    })
    .map(|get_response: GetResponse<Edge>| {
      let edge_address = get_response.address;
      // archive the edge with this address
      match hdk::remove_entry(&edge_address) {
        Ok(_) => Ok(edge_address),
        Err(e) => Err(e),
      }
    })
    // filter out errors
    .filter_map(Result::ok)
    .collect(); // returns vec of the edge addresses which were removed

  let archived_goal_members = archive_members_of_goal(&address)?; // returns vec of the goal_member addresses which were removed
  let archived_goal_votes = inner_fetch_goal_votes()?
    .into_iter()
    .filter(|get_response: &GetResponse<GoalVote>| {
      // check whether the parent_address or child_address is equal to the given address.
      // If so, the edge is connected to the goal being archived.
      get_response.entry.goal_address == address
    })
    .map(|get_response: GetResponse<GoalVote>| {
      let goal_vote_address = get_response.address;
      // archive the edge with this address
      match hdk::remove_entry(&goal_vote_address) {
        Ok(_) => Ok(goal_vote_address),
        Err(e) => Err(e),
      }
    })
    // filter out errors
    .filter_map(Result::ok)
    .collect();
  let archived_goal_comments = inner_fetch_goal_comments()?
    .into_iter()
    .filter(|get_response: &GetResponse<GoalComment>| {
      // check whether the parent_address or child_address is equal to the given address.
      // If so, the edge is connected to the goal being archived.
      get_response.entry.goal_address == address
    })
    .map(|get_response: GetResponse<GoalComment>| {
      let goal_comment_address = get_response.address;
      // archive the edge with this address
      match hdk::remove_entry(&goal_comment_address) {
        Ok(_) => Ok(goal_comment_address),
        Err(e) => Err(e),
      }
    })
    // filter out errors
    .filter_map(Result::ok)
    .collect(); // returns vec of the goal_member addresses which were removed
                // return the address of the archived goal for the UI to use
  let archived_entry_points = inner_fetch_entry_points()?
    .into_iter()
    .filter(|entry_point_response: &EntryPointResponse| {
      entry_point_response.entry_point.goal_address == address
    })
    .map(|entry_point_response: EntryPointResponse| {
      let entry_point_address = entry_point_response.entry_point_address;
      match hdk::remove_entry(&entry_point_address) {
        Ok(_) => Ok(entry_point_address),
        Err(e) => Err(e),
      }
    })
    // filter out errors
    .filter_map(Result::ok)
    .collect(); // returns vec of the entry_points addresses which were removed
                // return the address of the archived goal for the UI to use

  let archive_response = ArchiveGoalResponse {
    address,
    archived_edges,
    archived_goal_members,
    archived_goal_votes,
    archived_goal_comments,
    archived_entry_points,
  };
  // notify_goal_archived(archive_response.clone())?;

  Ok(archive_response)
}

pub fn archive_edge(address: Address) -> ExternResult<Address> {
  hdk::remove_entry(&address)?;
  // notify_edge_archived(address.clone())?;
  Ok(address)
}

pub fn add_member_of_goal(goal_member: GoalMember) -> ExternResult<GetResponse<GoalMember>> {
  let mut goal_member = goal_member;
  goal_member.user_edit_hash = Some(AGENT_ADDRESS.clone());
  let app_entry = Entry::App("goal_member".into(), goal_member.clone().into());
  let entry_address = hdk::commit_entry(&app_entry)?;

  // link new edge to the edges anchor
  let anchor_address = Entry::App(
    "anchor".into(),       // app entry type
    "goal_members".into(), // app entry value
  )
  .address();

  hdk::link_entries(
    &anchor_address,
    &app_entry.address(),
    "anchor->goal_member",
    "",
  )?;

  let get_response = GetResponse {
    entry: goal_member,
    address: entry_address,
  };
  // notify_goal_member(get_response.clone())?;

  Ok(get_response)
}

pub fn add_vote_of_goal(goal_vote: GoalVote) -> ExternResult<GetResponse<GoalVote>> {
  let app_entry = Entry::App("goal_vote".into(), goal_vote.clone().into());
  let entry_address = hdk::commit_entry(&app_entry)?;

  // link new edge to the edges anchor
  let anchor_address = Entry::App(
    "anchor".into(),     // app entry type
    "goal_votes".into(), // app entry value
  )
  .address();

  hdk::link_entries(
    &anchor_address,
    &app_entry.address(),
    "anchor->goal_vote",
    "",
  )?;

  let get_response = GetResponse {
    entry: goal_vote,
    address: entry_address,
  };
  // notify_goal_vote(get_response.clone())?;

  Ok(get_response)
}

pub fn update_goal_vote(
  goal_vote: GoalVote,
  address: Address,
) -> ExternResult<GetResponse<GoalVote>> {
  let app_entry = Entry::App("goal_vote".into(), goal_vote.clone().into());
  let _ = hdk::update_entry(app_entry, &address)?;

  // format the response as a GetResponse
  // pass the OLD address back and allow the UI to continue to use it
  let get_response = GetResponse {
    entry: goal_vote,
    address: address,
  };
  // notify_goal_vote(get_response.clone())?;

  Ok(get_response)
}

pub fn add_comment_of_goal(goal_comment: GoalComment) -> ExternResult<GetResponse<GoalComment>> {
  let app_entry = Entry::App("goal_comment".into(), goal_comment.clone().into());
  let entry_address = hdk::commit_entry(&app_entry)?;

  // link new edge to the edges anchor
  let anchor_address = Entry::App(
    "anchor".into(),        // app entry type
    "goal_comments".into(), // app entry value
  )
  .address();

  hdk::link_entries(
    &anchor_address,
    &app_entry.address(),
    "anchor->goal_comment",
    "",
  )?;

  let get_response = GetResponse {
    entry: goal_comment,
    address: entry_address,
  };
  // notify_goal_comment(get_response.clone())?;

  Ok(get_response)
}

pub fn update_goal_comment(
  goal_comment: GoalComment,
  address: Address,
) -> ExternResult<GetResponse<GoalComment>> {
  let app_entry = Entry::App("goal_comment".into(), goal_comment.clone().into());
  let _ = hdk::update_entry(app_entry, &address)?;

  // format the response as a GetResponse
  // pass the OLD address back and allow the UI to continue to use it
  let get_response = GetResponse {
    entry: goal_comment,
    address,
  };
  // notify_goal_comment(get_response.clone())?;

  Ok(get_response)
}

pub fn archive_members_of_goal(address: &Address) -> ExternResult<Vec<Address>> {
  inner_fetch_goal_members()?
    .into_iter()
    .filter(|get_response: &GetResponse<GoalMember>| {
      // check whether the parent_address or child_address is equal to the given address.
      // If so, the edge is connected to the goal being archived.
      get_response.entry.goal_address == *address
    })
    .map(|get_response: GetResponse<GoalMember>| {
      let goal_member_address = get_response.address;
      // archive the edge with this address
      match hdk::remove_entry(&goal_member_address) {
        Ok(_) => {
          notify_goal_member_archived(goal_member_address.clone())?;
          Ok(goal_member_address)
        }
        Err(e) => Err(e),
      }
    })
    // filter out errors
    .collect()
}

pub fn archive_member_of_goal(address: Address) -> ExternResult<Address> {
  hdk::remove_entry(&address)?;
  // notify_goal_member_archived(address.clone())?;
  Ok(address)
}

pub fn archive_vote_of_goal(address: Address) -> ExternResult<Address> {
  hdk::remove_entry(&address)?;
  // notify_goal_vote_archived(address.clone())?;
  Ok(address)
}

pub fn archive_comment_of_goal(address: Address) -> ExternResult<Address> {
  hdk::remove_entry(&address)?;
  // notify_goal_comment_archived(address.clone())?;
  Ok(address)
}

pub fn fetch_goal_members() -> ExternResult<Vec<GetResponse<GoalMember>>> {
  inner_fetch_goal_members()
}
pub fn fetch_goal_votes() -> ExternResult<Vec<GetResponse<GoalVote>>> {
  inner_fetch_goal_votes()
}
pub fn fetch_goal_comments() -> ExternResult<Vec<GetResponse<GoalComment>>> {
  inner_fetch_goal_comments()
}

pub fn fetch_members() -> ExternResult<Vec<Member>> {
  let anchor_address = Entry::App(
    "anchor".into(), // app entry type
    // app entry value. We'll use the value to specify what this anchor is for
    "members".into(),
  )
  .address();

  Ok(
    // return all the Member objects from the entries linked to the members anchor (drop entries with wrong type)
    hdk::utils::get_links_and_load_type(
      &anchor_address,
      LinkMatch::Exactly("anchor->member"), // the link type to match
      LinkMatch::Any,
    )?,
  )
}

// send the direct messages that will result in
// signals being emitted to the UI
// fn notify_all(message: DirectMessage) -> ExternResult<()> {
//   fetch_members()?.iter().for_each(|member| {
//     // useful for development purposes
//     // uncomment to send signals to oneself
//     // if member.address == AGENT_ADDRESS.to_string() {
//     //     signal_ui(&message);
//     // }

//     if member.address != AGENT_ADDRESS.to_string() {
//       hdk::debug(format!(
//         "Send a message to: {:?}",
//         &member.address.to_string()
//       ))
//       .ok();
//       hdk::send(
//         Address::from(member.address.clone()),
//         JsonString::from(message.clone()).into(),
//         1.into(),
//       )
//       .ok();
//     }
//   });
//   Ok(())
// }

// fn notify_member(member: Member) -> ExternResult<()> {
//   let message = DirectMessage::NewMemberNotification(NewMemberSignalPayload {
//     member: member.clone(),
//   });
//   notify_all(message)
// }

// fn notify_entry_point(entry_point_response: EntryPointResponse) -> ExternResult<()> {
//   let message = DirectMessage::EntryPointNotification(EntryPointSignalPayload {
//     entry_point: entry_point_response.clone(),
//   });
//   notify_all(message)
// }

// fn notify_entry_point_archived(address: Address) -> ExternResult<()> {
//   let message =
//     DirectMessage::EntryPointArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }

// fn notify_goal_maybe_with_edge(goal_maybe_with_edge: GoalMaybeWithEdge) -> ExternResult<()> {
//   let message = DirectMessage::GoalMaybeWithEdgeNotification(GoalMaybeWithEdgeSignalPayload {
//     goal: goal_maybe_with_edge.clone(),
//   });
//   notify_all(message)
// }

// // handles create and update cases
// fn notify_edge(edge: GetResponse<Edge>) -> ExternResult<()> {
//   let message = DirectMessage::EdgeNotification(EdgeSignalPayload { edge });
//   notify_all(message)
// }

// fn notify_edge_archived(address: Address) -> ExternResult<()> {
//   let message = DirectMessage::EdgeArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }

// fn notify_goal_archived(archived: ArchiveGoalResponse) -> ExternResult<()> {
//   let message = DirectMessage::GoalArchivedNotification(GoalArchivedSignalPayload { archived });
//   notify_all(message)
// }

// fn notify_goal_comment(goal_comment: GetResponse<GoalComment>) -> ExternResult<()> {
//   let message = DirectMessage::GoalCommentNotification(GoalCommentSignalPayload { goal_comment });
//   notify_all(message)
// }

// fn notify_goal_member(goal_member: GetResponse<GoalMember>) -> ExternResult<()> {
//   let message = DirectMessage::GoalMemberNotification(GoalMemberSignalPayload { goal_member });
//   notify_all(message)
// }

// fn notify_goal_vote(goal_vote: GetResponse<GoalVote>) -> ExternResult<()> {
//   let message = DirectMessage::GoalVoteNotification(GoalVoteSignalPayload { goal_vote });
//   notify_all(message)
// }

// fn notify_goal_comment_archived(address: Address) -> ExternResult<()> {
//   let message =
//     DirectMessage::GoalCommentArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }

// fn notify_goal_member_archived(address: Address) -> ExternResult<()> {
//   let message =
//     DirectMessage::GoalMemberArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }

// fn notify_goal_vote_archived(address: Address) -> ExternResult<()> {
//   let message = DirectMessage::GoalVoteArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }