table! {
    accounts (id) {
        id -> Unsigned<Integer>,
        account_type_id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
        created -> Datetime,
        modified -> Datetime,
        old_id -> Nullable<Varchar>,
    }
}

table! {
    account_types (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    atags (id) {
        id -> Integer,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    attachments (id) {
        id -> Bigint,
        name -> Varchar,
        created -> Datetime,
        modified -> Datetime,
        #[sql_name = "type"]
        type_ -> Varchar,
        subtype -> Varchar,
        size -> Integer,
        md5 -> Varchar,
        date -> Nullable<Datetime>,
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        author -> Nullable<Varchar>,
        copyright -> Nullable<Varchar>,
        path -> Nullable<Varchar>,
        embed -> Nullable<Text>,
        profile -> Varchar,
        width -> Nullable<Unsigned<Integer>>,
        height -> Nullable<Unsigned<Integer>>,
    }
}

table! {
    attachments_atags (attachment_id, atag_id) {
        attachment_id -> Bigint,
        atag_id -> Integer,
    }
}

table! {
    contacts (id) {
        id -> Char,
        owner_id -> Char,
        currency_id -> Unsigned<Integer>,
        created -> Datetime,
        modified -> Datetime,
        vendor_id -> Nullable<Unsigned<Integer>>,
        account_id -> Nullable<Unsigned<Integer>>,
        user_id -> Nullable<Char>,
        lead_source_id -> Nullable<Unsigned<Integer>>,
        honorific_title -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        department -> Nullable<Varchar>,
        home_phone -> Nullable<Varchar>,
        fax -> Nullable<Varchar>,
        birthdate -> Nullable<Date>,
        assistant_phone -> Nullable<Varchar>,
        email_opt_out -> Nullable<Bool>,
        email -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        other_phone -> Nullable<Varchar>,
        mobile -> Nullable<Varchar>,
        assistant -> Nullable<Varchar>,
        skype -> Nullable<Varchar>,
        secondary_email -> Nullable<Varchar>,
        twitter -> Nullable<Varchar>,
        reporting_to -> Nullable<Char>,
        street -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

table! {
    contacts_tags (contact_id, tag_id) {
        contact_id -> Char,
        tag_id -> Unsigned<Integer>,
    }
}

table! {
    currencies (id) {
        id -> Unsigned<Integer>,
        code -> Varchar,
        rate -> Float,
    }
}

table! {
    i18n (id) {
        id -> Bigint,
        locale -> Varchar,
        model -> Varchar,
        foreign_key -> Bigint,
        field -> Varchar,
        content -> Nullable<Text>,
    }
}

table! {
    labels (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    lead_sources (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    logs (id) {
        id -> Char,
        model -> Varchar,
        foreign_key -> Char,
        created -> Datetime,
        content -> Text,
    }
}

table! {
    milestones (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        duedate -> Datetime,
        description -> Nullable<Text>,
        project_id -> Unsigned<Integer>,
    }
}

table! {
    projects (id) {
        id -> Unsigned<Integer>,
        account_id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
        created -> Datetime,
        modified -> Datetime,
    }
}

table! {
    queued_jobs (id) {
        id -> Integer,
        job_type -> Varchar,
        data -> Nullable<Longtext>,
        job_group -> Nullable<Varchar>,
        reference -> Nullable<Varchar>,
        created -> Nullable<Datetime>,
        notbefore -> Nullable<Datetime>,
        fetched -> Nullable<Datetime>,
        completed -> Nullable<Datetime>,
        progress -> Nullable<Float>,
        failed -> Integer,
        failure_message -> Nullable<Text>,
        workerkey -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        priority -> Integer,
    }
}

table! {
    records (email, start) {
        created -> Datetime,
        modified -> Datetime,
        user -> Varchar,
        email -> Varchar,
        client -> Varchar,
        project -> Varchar,
        description -> Nullable<Varchar>,
        task -> Nullable<Varchar>,
        tags -> Nullable<Text>,
        start -> Datetime,
        end -> Datetime,
        duration -> Time,
        duration_decimal -> Float,
    }
}

table! {
    roles (id) {
        id -> Char,
        note -> Nullable<Text>,
        role_type_id -> Unsigned<Integer>,
        project_id -> Unsigned<Integer>,
        user_id -> Char,
    }
}

table! {
    role_types (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    social_accounts (id) {
        id -> Char,
        user_id -> Char,
        provider -> Varchar,
        username -> Nullable<Varchar>,
        reference -> Varchar,
        avatar -> Nullable<Varchar>,
        description -> Nullable<Text>,
        link -> Varchar,
        token -> Varchar,
        token_secret -> Nullable<Varchar>,
        token_expires -> Nullable<Datetime>,
        active -> Bool,
        data -> Text,
        created -> Datetime,
        modified -> Datetime,
    }
}

table! {
    tags (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slig -> Varchar,
    }
}

table! {
    tasks (id) {
        id -> Char,
        project_id -> Unsigned<Integer>,
        task_status_id -> Unsigned<Integer>,
        owner_id -> Char,
        created -> Datetime,
        modified -> Datetime,
        title -> Varchar,
        start -> Nullable<Date>,
        description -> Nullable<Longtext>,
        end -> Nullable<Date>,
        parent_id -> Nullable<Char>,
        milestone_id -> Nullable<Unsigned<Integer>>,
    }
}

table! {
    tasks_attachments (tasks_id, attachments_id) {
        tasks_id -> Char,
        attachments_id -> Bigint,
        order -> Nullable<Integer>,
    }
}

table! {
    tasks_labels (task_id, label_id) {
        task_id -> Char,
        label_id -> Unsigned<Integer>,
    }
}

table! {
    task_followers (id) {
        id -> Char,
        task_id -> Char,
        user_id -> Char,
        task_follower_type_id -> Unsigned<Integer>,
    }
}

table! {
    task_follower_types (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    task_messages (id) {
        id -> Char,
        task_id -> Char,
        user_id -> Char,
        created -> Datetime,
        message -> Longtext,
    }
}

table! {
    task_messages_attachments (task_message_id, attachment_id) {
        task_message_id -> Char,
        attachment_id -> Bigint,
        order -> Nullable<Integer>,
    }
}

table! {
    task_status (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    tickets (id) {
        id -> Char,
        project_id -> Unsigned<Integer>,
        ticket_status_id -> Unsigned<Integer>,
        created -> Datetime,
        modified -> Datetime,
        title -> Varchar,
        description -> Nullable<Longtext>,
    }
}

table! {
    tickets_attachments (ticket_id, attachment_id) {
        ticket_id -> Char,
        attachment_id -> Bigint,
        order -> Nullable<Integer>,
    }
}

table! {
    ticket_messages (id) {
        id -> Char,
        ticket_id -> Char,
        user_id -> Char,
        created -> Datetime,
        message -> Longtext,
    }
}

table! {
    ticket_messages_attachments (ticket_message_id, attachment_id) {
        ticket_message_id -> Char,
        attachment_id -> Bigint,
        order -> Nullable<Integer>,
    }
}

table! {
    ticket_status (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    time_records (id) {
        id -> Char,
        user_id -> Char,
        project_id -> Unsigned<Integer>,
        role_type_id -> Unsigned<Integer>,
        created -> Datetime,
        modified -> Datetime,
        start -> Datetime,
        end -> Nullable<Datetime>,
        duration -> Time,
        duration_decimal -> Float,
        is_active -> Bool,
        description -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Char,
        username -> Varchar,
        email -> Nullable<Varchar>,
        password -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        token -> Nullable<Varchar>,
        token_expires -> Nullable<Datetime>,
        api_token -> Nullable<Varchar>,
        activation_date -> Nullable<Datetime>,
        tos_date -> Nullable<Datetime>,
        active -> Bool,
        is_superuser -> Bool,
        role -> Nullable<Varchar>,
        created -> Datetime,
        modified -> Datetime,
        attachment_id -> Nullable<Varchar>,
    }
}

joinable!(accounts -> account_types (account_type_id));
joinable!(attachments_atags -> atags (atag_id));
joinable!(attachments_atags -> attachments (attachment_id));
joinable!(contacts -> currencies (currency_id));
joinable!(contacts_tags -> contacts (contact_id));
joinable!(contacts_tags -> tags (tag_id));
joinable!(milestones -> projects (project_id));
joinable!(projects -> accounts (account_id));
joinable!(roles -> projects (project_id));
joinable!(roles -> role_types (role_type_id));
joinable!(social_accounts -> users (user_id));
joinable!(task_followers -> task_follower_types (task_follower_type_id));
joinable!(task_followers -> tasks (task_id));
joinable!(task_messages -> tasks (task_id));
joinable!(task_messages_attachments -> attachments (attachment_id));
joinable!(task_messages_attachments -> task_messages (task_message_id));
joinable!(tasks -> projects (project_id));
joinable!(tasks -> task_status (task_status_id));
joinable!(tasks_attachments -> attachments (attachments_id));
joinable!(tasks_attachments -> tasks (tasks_id));
joinable!(tasks_labels -> labels (label_id));
joinable!(tasks_labels -> tasks (task_id));
joinable!(ticket_messages -> tickets (ticket_id));
joinable!(ticket_messages_attachments -> attachments (attachment_id));
joinable!(ticket_messages_attachments -> ticket_messages (ticket_message_id));
joinable!(tickets -> projects (project_id));
joinable!(tickets -> ticket_status (ticket_status_id));
joinable!(tickets_attachments -> attachments (attachment_id));
joinable!(tickets_attachments -> tickets (ticket_id));
joinable!(time_records -> projects (project_id));
joinable!(time_records -> role_types (role_type_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    account_types,
    atags,
    attachments,
    attachments_atags,
    contacts,
    contacts_tags,
    currencies,
    i18n,
    labels,
    lead_sources,
    logs,
    milestones,
    projects,
    queued_jobs,
    records,
    roles,
    role_types,
    social_accounts,
    tags,
    tasks,
    tasks_attachments,
    tasks_labels,
    task_followers,
    task_follower_types,
    task_messages,
    task_messages_attachments,
    task_status,
    tickets,
    tickets_attachments,
    ticket_messages,
    ticket_messages_attachments,
    ticket_status,
    time_records,
    users,
);
