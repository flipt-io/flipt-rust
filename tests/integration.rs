use flipt::auth::{token::TokenCreateRequest, token::TokenListRequest, AuthClient};
use flipt::evaluate::{EvaluateClient, EvaluateRequest as V2EvaluateRequest, Reason as V2Reason};
use flipt::Config;
use flipt::{
    api::{
        constraint::{
            ComparisonType, Constraint, ConstraintCreateRequest, ConstraintDeleteRequest, Operator,
        },
        distribution::DistributionCreateRequest,
        evaluation::{EvaluateRequest, Reason},
        flag::{FlagCreateRequest, FlagDeleteRequest, FlagType},
        namespace::{NamespaceCreateRequest, NamespaceDeleteRequest},
        rollout::{
            Rollout, RolloutCreateRequest, RolloutDeleteRequest, RolloutSegment, RolloutThreshold,
            RolloutType,
        },
        rule::{Rule, RuleCreateRequest, RuleDeleteRequest},
        segment::{Match, SegmentCreateRequest, SegmentDeleteRequest},
        variant::{Variant, VariantCreateRequest},
        ApiClient,
    },
    meta::MetaClient,
};

#[tokio::test]
#[cfg_attr(not(feature = "flipt_integration"), ignore)]
async fn integration_api() {
    let config = Config::new_from_env().expect("config");
    let client = ApiClient::new(config).expect("build client");

    const NAMESPACE_KEY: &str = "namespace-a";
    const BOOLEAN_FLAG_KEY: &str = "flag-boolean";
    const FLAG_KEY: &str = "flag-a";
    const VARIANT_KEY: &str = "variant-a";
    const SEGMENT_KEY: &str = "segment-a";

    let _ = client
        .flags()
        .delete(&FlagDeleteRequest {
            key: FLAG_KEY.into(),
            ..Default::default()
        })
        .await;
    let _ = client
        .segments()
        .delete(&SegmentDeleteRequest {
            key: SEGMENT_KEY.into(),
            ..Default::default()
        })
        .await;

    create_flag(&client, FLAG_KEY, FlagType::Variant).await;
    let variant = create_variant(&client, FLAG_KEY, VARIANT_KEY).await;
    create_segment(&client, SEGMENT_KEY).await;
    let constraint = create_constraint(&client, SEGMENT_KEY).await;
    let rule = create_rule(&client, FLAG_KEY, SEGMENT_KEY).await;
    create_distribution(&client, FLAG_KEY, &rule.id, &variant.id).await;
    evaluate(&client, FLAG_KEY).await;
    create_namespace(&client, NAMESPACE_KEY).await;

    // rollouts
    create_flag(&client, BOOLEAN_FLAG_KEY, FlagType::Boolean).await;
    let threshold_rollout = create_threshold_rollout(&client, BOOLEAN_FLAG_KEY).await;
    let segment_rollout = create_segment_rollout(&client, BOOLEAN_FLAG_KEY, SEGMENT_KEY).await;

    let evaluate_client = EvaluateClient::new(&client);
    boolean_evaluate(&evaluate_client, BOOLEAN_FLAG_KEY).await;
    variant_evaluate(&evaluate_client, FLAG_KEY).await;

    let _ = client
        .flags()
        .delete(&FlagDeleteRequest {
            key: FLAG_KEY.into(),
            ..Default::default()
        })
        .await;
    let _ = client
        .segments()
        .delete(&SegmentDeleteRequest {
            key: SEGMENT_KEY.into(),
            ..Default::default()
        })
        .await;
    let _ = client
        .constraints()
        .delete(&ConstraintDeleteRequest {
            segment_key: SEGMENT_KEY.into(),
            id: constraint.id,
            ..Default::default()
        })
        .await;
    let _ = client
        .rules()
        .delete(&RuleDeleteRequest {
            flag_key: FLAG_KEY.into(),
            id: rule.id,
            ..Default::default()
        })
        .await;
    let _ = client.namespaces().delete(&NamespaceDeleteRequest {
        key: NAMESPACE_KEY.into(),
    });
    let _ = client
        .flags()
        .delete(&FlagDeleteRequest {
            key: BOOLEAN_FLAG_KEY.into(),
            ..Default::default()
        })
        .await;

    delete_rollout(&client, BOOLEAN_FLAG_KEY, &threshold_rollout.id).await;
    delete_rollout(&client, BOOLEAN_FLAG_KEY, &segment_rollout.id).await;

    let _ = client
        .namespaces()
        .delete(&NamespaceDeleteRequest {
            key: NAMESPACE_KEY.into(),
            ..Default::default()
        })
        .await;

    async fn create_namespace(client: &ApiClient, key: &str) {
        let namespace = client
            .namespaces()
            .create(&NamespaceCreateRequest {
                key: key.into(),
                name: "Namespace".into(),
                ..Default::default()
            })
            .await
            .expect("create namespace");

        assert_eq!(namespace.key, key);
        assert_eq!(namespace.name, "Namespace");
        assert_eq!(namespace.description, "");
        assert!(!namespace.protected);
    }

    async fn create_threshold_rollout(client: &ApiClient, flag_key: &str) -> Rollout {
        let rollout = client
            .rollouts()
            .create(&RolloutCreateRequest {
                flag_key: flag_key.into(),
                rank: 1,
                threshold: Some(RolloutThreshold {
                    percentage: 50.0,
                    value: true,
                }),
                ..Default::default()
            })
            .await
            .expect("create threshold rollout");

        assert_eq!(rollout.id.is_empty(), false);
        assert_eq!(rollout.rank, 1);
        assert_eq!(rollout.description, "");
        assert_eq!(rollout.rollout_type, RolloutType::Threshold);

        rollout
    }

    async fn create_segment_rollout(
        client: &ApiClient,
        flag_key: &str,
        segment_key: &str,
    ) -> Rollout {
        let rollout = client
            .rollouts()
            .create(&RolloutCreateRequest {
                flag_key: flag_key.into(),
                rank: 2,
                segment: Some(RolloutSegment {
                    segment_key: segment_key.into(),
                    value: true,
                }),
                ..Default::default()
            })
            .await
            .expect("create segment rollout");

        assert_eq!(rollout.id.is_empty(), false);
        assert_eq!(rollout.rank, 2);
        assert_eq!(rollout.description, "");
        assert_eq!(rollout.rollout_type, RolloutType::Segment);

        rollout
    }

    async fn create_flag(client: &ApiClient, key: &str, flag_type: FlagType) {
        let flag = client
            .flags()
            .create(&FlagCreateRequest {
                key: key.into(),
                name: key.into(),
                enabled: true,
                r#type: Some(flag_type),
                ..Default::default()
            })
            .await
            .expect("create flag");

        assert_eq!(flag.key, key);
        assert_eq!(flag.name, key);
        assert_eq!(flag.r#type.unwrap(), flag_type);
        assert_eq!(flag.description, "");
        assert!(flag.enabled);
    }

    async fn create_variant(client: &ApiClient, flag_key: &str, key: &str) -> Variant {
        let variant = client
            .variants()
            .create(&VariantCreateRequest {
                flag_key: flag_key.into(),
                key: key.into(),
                name: "Variant".into(),
                ..Default::default()
            })
            .await
            .expect("create variant");

        assert_eq!(variant.key, key);
        assert_eq!(variant.name, "Variant");
        assert_eq!(variant.description, "");

        variant
    }

    async fn create_segment(client: &ApiClient, key: &str) {
        let segment = client
            .segments()
            .create(&SegmentCreateRequest {
                key: key.into(),
                name: "Segment".into(),
                ..Default::default()
            })
            .await
            .expect("create variant");
        assert_eq!(segment.key, key);
        assert_eq!(segment.name, "Segment");
        assert_eq!(segment.description, "");
        assert_eq!(segment.match_type, Match::All);
    }

    async fn create_constraint(client: &ApiClient, segment_key: &str) -> Constraint {
        let constraint = client
            .constraints()
            .create(&ConstraintCreateRequest {
                segment_key: segment_key.into(),
                property: "name".into(),
                value: "brett".into(),
                operator: Operator::Eq,
                comparison_type: ComparisonType::String,
                description: "desc".into(),
                ..Default::default()
            })
            .await
            .expect("create segment");

        assert!(constraint.id.len() != 0);
        assert_eq!(constraint.operator, Operator::Eq);
        assert_eq!(constraint.property, "name");
        assert_eq!(constraint.value, "brett");
        assert_eq!(constraint.comparison_type, ComparisonType::String);
        assert_eq!(constraint.description, "desc");

        constraint
    }

    async fn create_rule(client: &ApiClient, flag_key: &str, segment_key: &str) -> Rule {
        let rule = client
            .rules()
            .create(&RuleCreateRequest {
                rank: 1,
                flag_key: flag_key.into(),
                segment_key: segment_key.into(),
                ..Default::default()
            })
            .await
            .expect("create rule");

        assert!(rule.id.len() != 0);
        assert_eq!(rule.flag_key, flag_key);
        assert_eq!(rule.rank, 1u32);
        assert_eq!(rule.segment_key, "segment-a");
        assert_eq!(rule.distributions, (&[]).to_vec());

        rule
    }

    async fn create_distribution(
        client: &ApiClient,
        flag_key: &str,
        rule_id: &str,
        variant_id: &str,
    ) {
        let dist = client
            .distributions()
            .create(&DistributionCreateRequest {
                flag_key: flag_key.into(),
                rule_id: rule_id.into(),
                rollout: 100.0,
                variant_id: variant_id.into(),
                ..Default::default()
            })
            .await
            .expect("create distribution");

        assert!(dist.id.len() != 0);
        assert_eq!(dist.rule_id, rule_id);
        assert_eq!(dist.variant_id, variant_id);
        assert_eq!(dist.rollout, 100.0);
    }

    async fn evaluate(client: &ApiClient, flag_key: &str) {
        let eval = client
            .evaluation()
            .evaluate(&EvaluateRequest {
                entity_id: "abc".into(),
                context: std::collections::HashMap::from([(
                    String::from("name"),
                    String::from("brett"),
                )]),
                flag_key: flag_key.into(),
                ..Default::default()
            })
            .await
            .expect("eval");
        assert!(eval.is_match);
        assert_eq!(eval.reason, Reason::Match);
    }

    async fn boolean_evaluate(client: &EvaluateClient<'_>, flag_key: &str) {
        let boolean_evaluation = client
            .boolean(&V2EvaluateRequest {
                namespace_key: String::from("default"),
                flag_key: flag_key.into(),
                entity_id: String::from("foo"),
                ..Default::default()
            })
            .await
            .expect("boolean evaluation");

        assert_eq!(boolean_evaluation.enabled, true);
        assert_eq!(boolean_evaluation.reason, V2Reason::Default);
    }

    async fn variant_evaluate(client: &EvaluateClient<'_>, flag_key: &str) {
        let variant_evaluation = client
            .variant(&V2EvaluateRequest {
                namespace_key: String::from("default"),
                flag_key: flag_key.into(),
                entity_id: String::from("foo"),
                context: std::collections::HashMap::from([(
                    String::from("name"),
                    String::from("brett"),
                )]),
                ..Default::default()
            })
            .await
            .expect("variant evaluation");

        assert_eq!(variant_evaluation.is_match, true);
        assert_eq!(variant_evaluation.reason, V2Reason::Match);
        assert_eq!(variant_evaluation.segment_key, "segment-a");
        assert_eq!(variant_evaluation.variant_key, "variant-a");
        assert_eq!(variant_evaluation.variant_attachment, "");
    }

    async fn delete_rollout(client: &ApiClient, flag_key: &str, id: &str) {
        let _ = client
            .rollouts()
            .delete(&RolloutDeleteRequest {
                flag_key: flag_key.into(),
                id: id.into(),
                ..Default::default()
            })
            .await;
    }
}

#[tokio::test]
#[cfg_attr(not(feature = "flipt_integration"), ignore)]
async fn integration_auth() {
    let config = Config::new_from_env().expect("config");
    let client = AuthClient::new(config).expect("build client");

    let list = client
        .tokens()
        .list(&TokenListRequest::default())
        .await
        .expect("list tokens");

    for auth in list.authentications {
        if auth.metadata[flipt::auth::METADATA_LABEL_NAME] == "e2e" {
            let _ = client.tokens().delete(&auth.id).await;
        }
    }

    let token = client
        .tokens()
        .create(&TokenCreateRequest {
            name: "e2e".into(),
            description: "foobar".into(),
            ..Default::default()
        })
        .await
        .expect("create token");
    assert_ne!(token.authentication.id, "");
    assert_eq!(token.authentication.expires_at, None);

    let metadata = token.authentication.metadata;
    assert_eq!(
        metadata
            .get(&String::from(flipt::auth::METADATA_LABEL_NAME))
            .expect("name from metadata"),
        "e2e"
    );
    assert_eq!(
        metadata
            .get(&String::from(flipt::auth::METADATA_LABEL_DESCRIPTION))
            .expect("description from metadata"),
        "foobar"
    );

    let _ = client.tokens().delete(&token.authentication.id).await;

    let me = client.me().await.expect("me");
    assert_ne!(me.id, "");
}

#[tokio::test]
#[cfg_attr(not(feature = "flipt_integration"), ignore)]
async fn integration_meta() {
    let config = Config::new_from_env().expect("config");
    let client = MetaClient::new(config).expect("build client");

    let info = client.info().get().await.expect("info");
    assert_ne!(info.is_empty(), true);
}
