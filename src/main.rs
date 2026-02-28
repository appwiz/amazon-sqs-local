use std::sync::Arc;

use clap::Parser;

mod acm;
mod amplify;
mod apigateway;
mod appfabric;
mod appflow;
mod appmesh;
mod apprunner;
mod appsync;
mod athena;
mod autoscaling;
mod b2bi;
mod backup;
mod batch;
mod bedrock;
mod billingconductor;
mod braket;
mod budgets;
mod chime;
mod cleanrooms;
mod cloudformation;
mod cloudfront;
mod cloudhsm;
mod cloudmap;
mod cloudsearch;
mod cloudtrail;
mod cloudwatch;
mod cloudwatchlogs;
mod codeartifact;
mod codebuild;
mod codecatalyst;
mod codecommit;
mod codedeploy;
mod codepipeline;
mod cognito;
mod comprehend;
mod computeoptimizer;
mod config;
mod connect;
mod controltower;
mod costexplorer;
mod dataexchange;
mod datapipeline;
mod datasync;
mod datazone;
mod detective;
mod devicefarm;
mod devopsguru;
mod directconnect;
mod directoryservice;
mod dms;
mod documentdb;
mod drs;
mod dynamodb;
mod ec2;
mod ecr;
mod ecs;
mod efs;
mod eks;
mod elasticache;
mod elasticbeanstalk;
mod elastictranscoder;
mod elb;
mod emr;
mod entityresolution;
mod eventbridge;
mod finspace;
mod firehose;
mod firewallmanager;
mod fis;
mod forecast;
mod frauddetector;
mod fsx;
mod gamelift;
mod globalaccelerator;
mod glue;
mod groundstation;
mod guardduty;
mod health;
mod healthlake;
mod iam;
mod iamidentitycenter;
mod imagebuilder;
mod inspector;
mod iotcore;
mod iotevents;
mod iotfleetwise;
mod iotgreengrass;
mod iotsitewise;
mod iottwinmaker;
mod ivs;
mod kendra;
mod keyspaces;
mod kinesis;
mod kinesisvideostreams;
mod kms;
mod lakeformation;
mod lambda;
mod lex;
mod licensemanager;
mod lightsail;
mod location;
mod macie;
mod mainframemod;
mod managedblockchain;
mod managedflink;
mod managedgrafana;
mod managedprometheus;
mod mediaconvert;
mod medialive;
mod mediapackage;
mod mediastore;
mod memorydb;
mod migrationhub;
mod mq;
mod msk;
mod mwaa;
mod neptune;
mod networkfirewall;
mod opensearch;
mod organizations;
mod outposts;
mod personalize;
mod pinpoint;
mod polly;
mod proton;
mod qbusiness;
mod quicksight;
mod ram;
mod rds;
mod redshift;
mod rekognition;
mod route53;
mod s3;
mod sagemaker;
mod secretsmanager;
mod securityhub;
mod securitylake;
mod servicecatalog;
mod ses;
mod shield;
mod sns;
mod sqs;
mod ssm;
mod stepfunctions;
mod storagegateway;
mod swf;
mod textract;
mod timestream;
mod transcribe;
mod transferfamily;
mod translate;
mod trustedadvisor;
mod verifiedpermissions;
mod vpclattice;
mod waf;
mod workdocs;
mod workmail;
mod workspaces;
mod xray;

#[derive(Parser)]
#[command(
    name = "aws-inmemory-services",
    about = "Local in-memory AWS service emulator"
)]
struct Args {
    #[arg(long, default_value = "10034")]
    acm_port: u16,
    #[arg(long, default_value = "10154")]
    amplify_port: u16,
    #[arg(long, default_value = "4567")]
    apigateway_port: u16,
    #[arg(long, default_value = "10128")]
    appfabric_port: u16,
    #[arg(long, default_value = "10112")]
    appflow_port: u16,
    #[arg(long, default_value = "10158")]
    appmesh_port: u16,
    #[arg(long, default_value = "10006")]
    apprunner_port: u16,
    #[arg(long, default_value = "9700")]
    appsync_port: u16,
    #[arg(long, default_value = "10050")]
    athena_port: u16,
    #[arg(long, default_value = "10011")]
    autoscaling_port: u16,
    #[arg(long, default_value = "10116")]
    b2bi_port: u16,
    #[arg(long, default_value = "10146")]
    backup_port: u16,
    #[arg(long, default_value = "10007")]
    batch_port: u16,
    #[arg(long, default_value = "10093")]
    bedrock_port: u16,
    #[arg(long, default_value = "10129")]
    billingconductor_port: u16,
    #[arg(long, default_value = "10150")]
    braket_port: u16,
    #[arg(long, default_value = "10130")]
    budgets_port: u16,
    #[arg(long, default_value = "10123")]
    chime_port: u16,
    #[arg(long, default_value = "10061")]
    cleanrooms_port: u16,
    #[arg(long, default_value = "10070")]
    cloudformation_port: u16,
    #[arg(long, default_value = "10021")]
    cloudfront_port: u16,
    #[arg(long, default_value = "10044")]
    cloudhsm_port: u16,
    #[arg(long, default_value = "10024")]
    cloudmap_port: u16,
    #[arg(long, default_value = "10051")]
    cloudsearch_port: u16,
    #[arg(long, default_value = "10071")]
    cloudtrail_port: u16,
    #[arg(long, default_value = "10067")]
    cloudwatch_port: u16,
    #[arg(long, default_value = "9201")]
    cloudwatchlogs_port: u16,
    #[arg(long, default_value = "10083")]
    codeartifact_port: u16,
    #[arg(long, default_value = "10084")]
    codebuild_port: u16,
    #[arg(long, default_value = "10082")]
    codecatalyst_port: u16,
    #[arg(long, default_value = "10085")]
    codecommit_port: u16,
    #[arg(long, default_value = "10086")]
    codedeploy_port: u16,
    #[arg(long, default_value = "10087")]
    codepipeline_port: u16,
    #[arg(long, default_value = "9229")]
    cognito_port: u16,
    #[arg(long, default_value = "10094")]
    comprehend_port: u16,
    #[arg(long, default_value = "10072")]
    computeoptimizer_port: u16,
    #[arg(long, default_value = "9500")]
    config_port: u16,
    #[arg(long, default_value = "10124")]
    connect_port: u16,
    #[arg(long, default_value = "10073")]
    controltower_port: u16,
    #[arg(long, default_value = "10131")]
    costexplorer_port: u16,
    #[arg(long, default_value = "10062")]
    dataexchange_port: u16,
    #[arg(long, default_value = "10063")]
    datapipeline_port: u16,
    #[arg(long, default_value = "10138")]
    datasync_port: u16,
    #[arg(long, default_value = "10052")]
    datazone_port: u16,
    #[arg(long, default_value = "10040")]
    detective_port: u16,
    #[arg(long, default_value = "10155")]
    devicefarm_port: u16,
    #[arg(long, default_value = "10106")]
    devopsguru_port: u16,
    #[arg(long, default_value = "10025")]
    directconnect_port: u16,
    #[arg(long, default_value = "10043")]
    directoryservice_port: u16,
    #[arg(long, default_value = "10018")]
    dms_port: u16,
    #[arg(long, default_value = "10013")]
    documentdb_port: u16,
    #[arg(long, default_value = "10149")]
    drs_port: u16,
    #[arg(long, default_value = "8000")]
    dynamodb_port: u16,
    #[arg(long, default_value = "10001")]
    ec2_port: u16,
    #[arg(long, default_value = "10002")]
    ecr_port: u16,
    #[arg(long, default_value = "10003")]
    ecs_port: u16,
    #[arg(long, default_value = "9600")]
    efs_port: u16,
    #[arg(long, default_value = "10004")]
    eks_port: u16,
    #[arg(long, default_value = "10014")]
    elasticache_port: u16,
    #[arg(long, default_value = "10008")]
    elasticbeanstalk_port: u16,
    #[arg(long, default_value = "10132")]
    elastictranscoder_port: u16,
    #[arg(long, default_value = "10027")]
    elb_port: u16,
    #[arg(long, default_value = "10053")]
    emr_port: u16,
    #[arg(long, default_value = "10064")]
    entityresolution_port: u16,
    #[arg(long, default_value = "9195")]
    eventbridge_port: u16,
    #[arg(long, default_value = "10054")]
    finspace_port: u16,
    #[arg(long, default_value = "4573")]
    firehose_port: u16,
    #[arg(long, default_value = "10047")]
    firewallmanager_port: u16,
    #[arg(long, default_value = "10088")]
    fis_port: u16,
    #[arg(long, default_value = "10095")]
    forecast_port: u16,
    #[arg(long, default_value = "10096")]
    frauddetector_port: u16,
    #[arg(long, default_value = "10147")]
    fsx_port: u16,
    #[arg(long, default_value = "10156")]
    gamelift_port: u16,
    #[arg(long, default_value = "10026")]
    globalaccelerator_port: u16,
    #[arg(long, default_value = "10065")]
    glue_port: u16,
    #[arg(long, default_value = "10151")]
    groundstation_port: u16,
    #[arg(long, default_value = "10037")]
    guardduty_port: u16,
    #[arg(long, default_value = "10074")]
    health_port: u16,
    #[arg(long, default_value = "10107")]
    healthlake_port: u16,
    #[arg(long, default_value = "10033")]
    iam_port: u16,
    #[arg(long, default_value = "10049")]
    iamidentitycenter_port: u16,
    #[arg(long, default_value = "10010")]
    imagebuilder_port: u16,
    #[arg(long, default_value = "10038")]
    inspector_port: u16,
    #[arg(long, default_value = "10117")]
    iotcore_port: u16,
    #[arg(long, default_value = "10118")]
    iotevents_port: u16,
    #[arg(long, default_value = "10119")]
    iotfleetwise_port: u16,
    #[arg(long, default_value = "10120")]
    iotgreengrass_port: u16,
    #[arg(long, default_value = "10121")]
    iotsitewise_port: u16,
    #[arg(long, default_value = "10122")]
    iottwinmaker_port: u16,
    #[arg(long, default_value = "10133")]
    ivs_port: u16,
    #[arg(long, default_value = "10097")]
    kendra_port: u16,
    #[arg(long, default_value = "10015")]
    keyspaces_port: u16,
    #[arg(long, default_value = "4568")]
    kinesis_port: u16,
    #[arg(long, default_value = "10055")]
    kinesisvideostreams_port: u16,
    #[arg(long, default_value = "7600")]
    kms_port: u16,
    #[arg(long, default_value = "10066")]
    lakeformation_port: u16,
    #[arg(long, default_value = "9001")]
    lambda_port: u16,
    #[arg(long, default_value = "10098")]
    lex_port: u16,
    #[arg(long, default_value = "10075")]
    licensemanager_port: u16,
    #[arg(long, default_value = "10005")]
    lightsail_port: u16,
    #[arg(long, default_value = "10153")]
    location_port: u16,
    #[arg(long, default_value = "10039")]
    macie_port: u16,
    #[arg(long, default_value = "10139")]
    mainframemod_port: u16,
    #[arg(long, default_value = "10157")]
    managedblockchain_port: u16,
    #[arg(long, default_value = "10056")]
    managedflink_port: u16,
    #[arg(long, default_value = "10068")]
    managedgrafana_port: u16,
    #[arg(long, default_value = "10069")]
    managedprometheus_port: u16,
    #[arg(long, default_value = "10134")]
    mediaconvert_port: u16,
    #[arg(long, default_value = "10135")]
    medialive_port: u16,
    #[arg(long, default_value = "10136")]
    mediapackage_port: u16,
    #[arg(long, default_value = "10137")]
    mediastore_port: u16,
    #[arg(long, default_value = "6379")]
    memorydb_port: u16,
    #[arg(long, default_value = "10140")]
    migrationhub_port: u16,
    #[arg(long, default_value = "10113")]
    mq_port: u16,
    #[arg(long, default_value = "10057")]
    msk_port: u16,
    #[arg(long, default_value = "10114")]
    mwaa_port: u16,
    #[arg(long, default_value = "10016")]
    neptune_port: u16,
    #[arg(long, default_value = "10048")]
    networkfirewall_port: u16,
    #[arg(long, default_value = "10058")]
    opensearch_port: u16,
    #[arg(long, default_value = "10076")]
    organizations_port: u16,
    #[arg(long, default_value = "10009")]
    outposts_port: u16,
    #[arg(long, default_value = "10099")]
    personalize_port: u16,
    #[arg(long, default_value = "10125")]
    pinpoint_port: u16,
    #[arg(long, default_value = "10100")]
    polly_port: u16,
    #[arg(long, default_value = "10077")]
    proton_port: u16,
    #[arg(long, default_value = "10108")]
    qbusiness_port: u16,
    #[arg(long, default_value = "10059")]
    quicksight_port: u16,
    #[arg(long, default_value = "10045")]
    ram_port: u16,
    #[arg(long, default_value = "10012")]
    rds_port: u16,
    #[arg(long, default_value = "10060")]
    redshift_port: u16,
    #[arg(long, default_value = "10101")]
    rekognition_port: u16,
    #[arg(long, default_value = "10022")]
    route53_port: u16,
    #[arg(long, default_value = "9000")]
    s3_port: u16,
    #[arg(long, default_value = "10102")]
    sagemaker_port: u16,
    #[arg(long, default_value = "7700")]
    secretsmanager_port: u16,
    #[arg(long, default_value = "10046")]
    securityhub_port: u16,
    #[arg(long, default_value = "10041")]
    securitylake_port: u16,
    #[arg(long, default_value = "9400")]
    servicecatalog_port: u16,
    #[arg(long, default_value = "9300")]
    ses_port: u16,
    #[arg(long, default_value = "10036")]
    shield_port: u16,
    #[arg(long, default_value = "9911")]
    sns_port: u16,
    #[arg(long, default_value = "9324")]
    sqs_port: u16,
    #[arg(long, default_value = "9100")]
    ssm_port: u16,
    #[arg(long, default_value = "8083")]
    stepfunctions_port: u16,
    #[arg(long, default_value = "10148")]
    storagegateway_port: u16,
    #[arg(long, default_value = "10115")]
    swf_port: u16,
    #[arg(long, default_value = "10103")]
    textract_port: u16,
    #[arg(long, default_value = "10017")]
    timestream_port: u16,
    #[arg(long, default_value = "10104")]
    transcribe_port: u16,
    #[arg(long, default_value = "10141")]
    transferfamily_port: u16,
    #[arg(long, default_value = "10105")]
    translate_port: u16,
    #[arg(long, default_value = "10078")]
    trustedadvisor_port: u16,
    #[arg(long, default_value = "10042")]
    verifiedpermissions_port: u16,
    #[arg(long, default_value = "10023")]
    vpclattice_port: u16,
    #[arg(long, default_value = "10035")]
    waf_port: u16,
    #[arg(long, default_value = "10126")]
    workdocs_port: u16,
    #[arg(long, default_value = "10127")]
    workmail_port: u16,
    #[arg(long, default_value = "10152")]
    workspaces_port: u16,
    #[arg(long, default_value = "10089")]
    xray_port: u16,
    #[arg(long, default_value = "us-east-1")]
    region: String,
    #[arg(long, default_value = "000000000000")]
    account_id: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let acm_state = Arc::new(acm::state::ACMState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let amplify_state = Arc::new(amplify::state::AmplifyState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let apigateway_state = Arc::new(apigateway::state::ApiGatewayState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let appfabric_state = Arc::new(appfabric::state::AppfabricState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let appflow_state = Arc::new(appflow::state::AppflowState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let appmesh_state = Arc::new(appmesh::state::AppmeshState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let apprunner_state = Arc::new(apprunner::state::ApprunnerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let appsync_state = Arc::new(appsync::state::AppSyncState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let athena_state = Arc::new(athena::state::AthenaState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let autoscaling_state = Arc::new(autoscaling::state::AutoscalingState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let b2bi_state = Arc::new(b2bi::state::B2biState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let backup_state = Arc::new(backup::state::BackupState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let batch_state = Arc::new(batch::state::BatchState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let bedrock_state = Arc::new(bedrock::state::BedrockState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let billingconductor_state = Arc::new(billingconductor::state::BillingconductorState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let braket_state = Arc::new(braket::state::BraketState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let budgets_state = Arc::new(budgets::state::BudgetsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let chime_state = Arc::new(chime::state::ChimeState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cleanrooms_state = Arc::new(cleanrooms::state::CleanroomsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cloudformation_state = Arc::new(cloudformation::state::CloudformationState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cloudfront_state = Arc::new(cloudfront::state::CloudfrontState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cloudhsm_state = Arc::new(cloudhsm::state::CloudhsmState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cloudmap_state = Arc::new(cloudmap::state::CloudmapState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cloudsearch_state = Arc::new(cloudsearch::state::CloudsearchState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cloudtrail_state = Arc::new(cloudtrail::state::CloudtrailState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cloudwatch_state = Arc::new(cloudwatch::state::CloudwatchState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cloudwatchlogs_state = Arc::new(cloudwatchlogs::state::CwlState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let codeartifact_state = Arc::new(codeartifact::state::CodeartifactState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let codebuild_state = Arc::new(codebuild::state::CodebuildState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let codecatalyst_state = Arc::new(codecatalyst::state::CodecatalystState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let codecommit_state = Arc::new(codecommit::state::CodecommitState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let codedeploy_state = Arc::new(codedeploy::state::CodedeployState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let codepipeline_state = Arc::new(codepipeline::state::CodepipelineState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let cognito_state = Arc::new(cognito::state::CognitoState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let comprehend_state = Arc::new(comprehend::state::ComprehendState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let computeoptimizer_state = Arc::new(computeoptimizer::state::ComputeoptimizerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let config_state = Arc::new(config::state::ConfigState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let connect_state = Arc::new(connect::state::ConnectState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let controltower_state = Arc::new(controltower::state::ControltowerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let costexplorer_state = Arc::new(costexplorer::state::CostexplorerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let dataexchange_state = Arc::new(dataexchange::state::DataexchangeState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let datapipeline_state = Arc::new(datapipeline::state::DatapipelineState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let datasync_state = Arc::new(datasync::state::DatasyncState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let datazone_state = Arc::new(datazone::state::DatazoneState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let detective_state = Arc::new(detective::state::DetectiveState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let devicefarm_state = Arc::new(devicefarm::state::DevicefarmState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let devopsguru_state = Arc::new(devopsguru::state::DevopsguruState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let directconnect_state = Arc::new(directconnect::state::DirectconnectState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let directoryservice_state = Arc::new(directoryservice::state::DirectoryserviceState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let dms_state = Arc::new(dms::state::DMSState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let documentdb_state = Arc::new(documentdb::state::DocumentdbState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let drs_state = Arc::new(drs::state::DRSState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let dynamodb_state = Arc::new(dynamodb::state::DynamoDbState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let ec2_state = Arc::new(ec2::state::EC2State::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let ecr_state = Arc::new(ecr::state::ECRState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let ecs_state = Arc::new(ecs::state::ECSState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let efs_state = Arc::new(efs::state::EfsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let eks_state = Arc::new(eks::state::EKSState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let elasticache_state = Arc::new(elasticache::state::ElasticacheState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let elasticbeanstalk_state = Arc::new(elasticbeanstalk::state::ElasticbeanstalkState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let elastictranscoder_state = Arc::new(elastictranscoder::state::ElastictranscoderState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let elb_state = Arc::new(elb::state::ELBState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let emr_state = Arc::new(emr::state::EMRState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let entityresolution_state = Arc::new(entityresolution::state::EntityresolutionState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let eventbridge_state = Arc::new(eventbridge::state::EventBridgeState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let finspace_state = Arc::new(finspace::state::FinspaceState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let firehose_state = Arc::new(firehose::state::FirehoseState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let firewallmanager_state = Arc::new(firewallmanager::state::FirewallmanagerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let fis_state = Arc::new(fis::state::FISState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let forecast_state = Arc::new(forecast::state::ForecastState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let frauddetector_state = Arc::new(frauddetector::state::FrauddetectorState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let fsx_state = Arc::new(fsx::state::FSXState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let gamelift_state = Arc::new(gamelift::state::GameliftState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let globalaccelerator_state = Arc::new(globalaccelerator::state::GlobalacceleratorState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let glue_state = Arc::new(glue::state::GlueState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let groundstation_state = Arc::new(groundstation::state::GroundstationState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let guardduty_state = Arc::new(guardduty::state::GuarddutyState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let health_state = Arc::new(health::state::HealthState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let healthlake_state = Arc::new(healthlake::state::HealthlakeState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let iam_state = Arc::new(iam::state::IAMState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let iamidentitycenter_state = Arc::new(iamidentitycenter::state::IamidentitycenterState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let imagebuilder_state = Arc::new(imagebuilder::state::ImagebuilderState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let inspector_state = Arc::new(inspector::state::InspectorState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let iotcore_state = Arc::new(iotcore::state::IotcoreState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let iotevents_state = Arc::new(iotevents::state::IoteventsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let iotfleetwise_state = Arc::new(iotfleetwise::state::IotfleetwiseState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let iotgreengrass_state = Arc::new(iotgreengrass::state::IotgreengrassState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let iotsitewise_state = Arc::new(iotsitewise::state::IotsitewiseState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let iottwinmaker_state = Arc::new(iottwinmaker::state::IottwinmakerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let ivs_state = Arc::new(ivs::state::IVSState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let kendra_state = Arc::new(kendra::state::KendraState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let keyspaces_state = Arc::new(keyspaces::state::KeyspacesState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let kinesis_state = Arc::new(kinesis::state::KinesisState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let kinesisvideostreams_state = Arc::new(kinesisvideostreams::state::KinesisvideostreamsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let kms_state = Arc::new(kms::state::KmsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let lakeformation_state = Arc::new(lakeformation::state::LakeformationState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let lambda_state = Arc::new(lambda::state::LambdaState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let lex_state = Arc::new(lex::state::LexState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let licensemanager_state = Arc::new(licensemanager::state::LicensemanagerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let lightsail_state = Arc::new(lightsail::state::LightsailState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let location_state = Arc::new(location::state::LocationState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let macie_state = Arc::new(macie::state::MacieState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let mainframemod_state = Arc::new(mainframemod::state::MainframemodState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let managedblockchain_state = Arc::new(managedblockchain::state::ManagedblockchainState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let managedflink_state = Arc::new(managedflink::state::ManagedflinkState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let managedgrafana_state = Arc::new(managedgrafana::state::ManagedgrafanaState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let managedprometheus_state = Arc::new(managedprometheus::state::ManagedprometheusState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let mediaconvert_state = Arc::new(mediaconvert::state::MediaconvertState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let medialive_state = Arc::new(medialive::state::MedialiveState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let mediapackage_state = Arc::new(mediapackage::state::MediapackageState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let mediastore_state = Arc::new(mediastore::state::MediastoreState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let memorydb_state = Arc::new(memorydb::state::MemoryDbState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let migrationhub_state = Arc::new(migrationhub::state::MigrationhubState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let mq_state = Arc::new(mq::state::MQState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let msk_state = Arc::new(msk::state::MSKState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let mwaa_state = Arc::new(mwaa::state::MwaaState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let neptune_state = Arc::new(neptune::state::NeptuneState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let networkfirewall_state = Arc::new(networkfirewall::state::NetworkfirewallState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let opensearch_state = Arc::new(opensearch::state::OpensearchState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let organizations_state = Arc::new(organizations::state::OrganizationsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let outposts_state = Arc::new(outposts::state::OutpostsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let personalize_state = Arc::new(personalize::state::PersonalizeState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let pinpoint_state = Arc::new(pinpoint::state::PinpointState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let polly_state = Arc::new(polly::state::PollyState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let proton_state = Arc::new(proton::state::ProtonState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let qbusiness_state = Arc::new(qbusiness::state::QbusinessState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let quicksight_state = Arc::new(quicksight::state::QuicksightState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let ram_state = Arc::new(ram::state::RAMState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let rds_state = Arc::new(rds::state::RDSState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let redshift_state = Arc::new(redshift::state::RedshiftState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let rekognition_state = Arc::new(rekognition::state::RekognitionState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let route53_state = Arc::new(route53::state::Route53State::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let s3_state = Arc::new(s3::state::S3State::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let sagemaker_state = Arc::new(sagemaker::state::SagemakerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let secretsmanager_state = Arc::new(secretsmanager::state::SecretsManagerState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let securityhub_state = Arc::new(securityhub::state::SecurityhubState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let securitylake_state = Arc::new(securitylake::state::SecuritylakeState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let servicecatalog_state = Arc::new(servicecatalog::state::ServiceCatalogState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let ses_state = Arc::new(ses::state::SesState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let shield_state = Arc::new(shield::state::ShieldState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let sns_state = Arc::new(sns::state::SnsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let sqs_state = Arc::new(sqs::state::SqsState::new(
        args.account_id.clone(),
        args.region.clone(),
        args.sqs_port,
    ));
    let ssm_state = Arc::new(ssm::state::SsmState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let stepfunctions_state = Arc::new(stepfunctions::state::SfnState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let storagegateway_state = Arc::new(storagegateway::state::StoragegatewayState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let swf_state = Arc::new(swf::state::SwfState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let textract_state = Arc::new(textract::state::TextractState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let timestream_state = Arc::new(timestream::state::TimestreamState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let transcribe_state = Arc::new(transcribe::state::TranscribeState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let transferfamily_state = Arc::new(transferfamily::state::TransferfamilyState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let translate_state = Arc::new(translate::state::TranslateState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let trustedadvisor_state = Arc::new(trustedadvisor::state::TrustedadvisorState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let verifiedpermissions_state = Arc::new(verifiedpermissions::state::VerifiedpermissionsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let vpclattice_state = Arc::new(vpclattice::state::VpclatticeState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let waf_state = Arc::new(waf::state::WAFState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let workdocs_state = Arc::new(workdocs::state::WorkdocsState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let workmail_state = Arc::new(workmail::state::WorkmailState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let workspaces_state = Arc::new(workspaces::state::WorkspacesState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));
    let xray_state = Arc::new(xray::state::XrayState::new(
        args.account_id.clone(),
        args.region.clone(),
    ));

    let acm_app = acm::server::create_router(acm_state);
    let amplify_app = amplify::server::create_router(amplify_state);
    let apigateway_app = apigateway::server::create_router(apigateway_state);
    let appfabric_app = appfabric::server::create_router(appfabric_state);
    let appflow_app = appflow::server::create_router(appflow_state);
    let appmesh_app = appmesh::server::create_router(appmesh_state);
    let apprunner_app = apprunner::server::create_router(apprunner_state);
    let appsync_app = appsync::server::create_router(appsync_state);
    let athena_app = athena::server::create_router(athena_state);
    let autoscaling_app = autoscaling::server::create_router(autoscaling_state);
    let b2bi_app = b2bi::server::create_router(b2bi_state);
    let backup_app = backup::server::create_router(backup_state);
    let batch_app = batch::server::create_router(batch_state);
    let bedrock_app = bedrock::server::create_router(bedrock_state);
    let billingconductor_app = billingconductor::server::create_router(billingconductor_state);
    let braket_app = braket::server::create_router(braket_state);
    let budgets_app = budgets::server::create_router(budgets_state);
    let chime_app = chime::server::create_router(chime_state);
    let cleanrooms_app = cleanrooms::server::create_router(cleanrooms_state);
    let cloudformation_app = cloudformation::server::create_router(cloudformation_state);
    let cloudfront_app = cloudfront::server::create_router(cloudfront_state);
    let cloudhsm_app = cloudhsm::server::create_router(cloudhsm_state);
    let cloudmap_app = cloudmap::server::create_router(cloudmap_state);
    let cloudsearch_app = cloudsearch::server::create_router(cloudsearch_state);
    let cloudtrail_app = cloudtrail::server::create_router(cloudtrail_state);
    let cloudwatch_app = cloudwatch::server::create_router(cloudwatch_state);
    let cloudwatchlogs_app = cloudwatchlogs::server::create_router(cloudwatchlogs_state);
    let codeartifact_app = codeartifact::server::create_router(codeartifact_state);
    let codebuild_app = codebuild::server::create_router(codebuild_state);
    let codecatalyst_app = codecatalyst::server::create_router(codecatalyst_state);
    let codecommit_app = codecommit::server::create_router(codecommit_state);
    let codedeploy_app = codedeploy::server::create_router(codedeploy_state);
    let codepipeline_app = codepipeline::server::create_router(codepipeline_state);
    let cognito_app = cognito::server::create_router(cognito_state);
    let comprehend_app = comprehend::server::create_router(comprehend_state);
    let computeoptimizer_app = computeoptimizer::server::create_router(computeoptimizer_state);
    let config_app = config::server::create_router(config_state);
    let connect_app = connect::server::create_router(connect_state);
    let controltower_app = controltower::server::create_router(controltower_state);
    let costexplorer_app = costexplorer::server::create_router(costexplorer_state);
    let dataexchange_app = dataexchange::server::create_router(dataexchange_state);
    let datapipeline_app = datapipeline::server::create_router(datapipeline_state);
    let datasync_app = datasync::server::create_router(datasync_state);
    let datazone_app = datazone::server::create_router(datazone_state);
    let detective_app = detective::server::create_router(detective_state);
    let devicefarm_app = devicefarm::server::create_router(devicefarm_state);
    let devopsguru_app = devopsguru::server::create_router(devopsguru_state);
    let directconnect_app = directconnect::server::create_router(directconnect_state);
    let directoryservice_app = directoryservice::server::create_router(directoryservice_state);
    let dms_app = dms::server::create_router(dms_state);
    let documentdb_app = documentdb::server::create_router(documentdb_state);
    let drs_app = drs::server::create_router(drs_state);
    let dynamodb_app = dynamodb::server::create_router(dynamodb_state);
    let ec2_app = ec2::server::create_router(ec2_state);
    let ecr_app = ecr::server::create_router(ecr_state);
    let ecs_app = ecs::server::create_router(ecs_state);
    let efs_app = efs::server::create_router(efs_state);
    let eks_app = eks::server::create_router(eks_state);
    let elasticache_app = elasticache::server::create_router(elasticache_state);
    let elasticbeanstalk_app = elasticbeanstalk::server::create_router(elasticbeanstalk_state);
    let elastictranscoder_app = elastictranscoder::server::create_router(elastictranscoder_state);
    let elb_app = elb::server::create_router(elb_state);
    let emr_app = emr::server::create_router(emr_state);
    let entityresolution_app = entityresolution::server::create_router(entityresolution_state);
    let eventbridge_app = eventbridge::server::create_router(eventbridge_state);
    let finspace_app = finspace::server::create_router(finspace_state);
    let firehose_app = firehose::server::create_router(firehose_state);
    let firewallmanager_app = firewallmanager::server::create_router(firewallmanager_state);
    let fis_app = fis::server::create_router(fis_state);
    let forecast_app = forecast::server::create_router(forecast_state);
    let frauddetector_app = frauddetector::server::create_router(frauddetector_state);
    let fsx_app = fsx::server::create_router(fsx_state);
    let gamelift_app = gamelift::server::create_router(gamelift_state);
    let globalaccelerator_app = globalaccelerator::server::create_router(globalaccelerator_state);
    let glue_app = glue::server::create_router(glue_state);
    let groundstation_app = groundstation::server::create_router(groundstation_state);
    let guardduty_app = guardduty::server::create_router(guardduty_state);
    let health_app = health::server::create_router(health_state);
    let healthlake_app = healthlake::server::create_router(healthlake_state);
    let iam_app = iam::server::create_router(iam_state);
    let iamidentitycenter_app = iamidentitycenter::server::create_router(iamidentitycenter_state);
    let imagebuilder_app = imagebuilder::server::create_router(imagebuilder_state);
    let inspector_app = inspector::server::create_router(inspector_state);
    let iotcore_app = iotcore::server::create_router(iotcore_state);
    let iotevents_app = iotevents::server::create_router(iotevents_state);
    let iotfleetwise_app = iotfleetwise::server::create_router(iotfleetwise_state);
    let iotgreengrass_app = iotgreengrass::server::create_router(iotgreengrass_state);
    let iotsitewise_app = iotsitewise::server::create_router(iotsitewise_state);
    let iottwinmaker_app = iottwinmaker::server::create_router(iottwinmaker_state);
    let ivs_app = ivs::server::create_router(ivs_state);
    let kendra_app = kendra::server::create_router(kendra_state);
    let keyspaces_app = keyspaces::server::create_router(keyspaces_state);
    let kinesis_app = kinesis::server::create_router(kinesis_state);
    let kinesisvideostreams_app = kinesisvideostreams::server::create_router(kinesisvideostreams_state);
    let kms_app = kms::server::create_router(kms_state);
    let lakeformation_app = lakeformation::server::create_router(lakeformation_state);
    let lambda_app = lambda::server::create_router(lambda_state);
    let lex_app = lex::server::create_router(lex_state);
    let licensemanager_app = licensemanager::server::create_router(licensemanager_state);
    let lightsail_app = lightsail::server::create_router(lightsail_state);
    let location_app = location::server::create_router(location_state);
    let macie_app = macie::server::create_router(macie_state);
    let mainframemod_app = mainframemod::server::create_router(mainframemod_state);
    let managedblockchain_app = managedblockchain::server::create_router(managedblockchain_state);
    let managedflink_app = managedflink::server::create_router(managedflink_state);
    let managedgrafana_app = managedgrafana::server::create_router(managedgrafana_state);
    let managedprometheus_app = managedprometheus::server::create_router(managedprometheus_state);
    let mediaconvert_app = mediaconvert::server::create_router(mediaconvert_state);
    let medialive_app = medialive::server::create_router(medialive_state);
    let mediapackage_app = mediapackage::server::create_router(mediapackage_state);
    let mediastore_app = mediastore::server::create_router(mediastore_state);
    let memorydb_app = memorydb::server::create_router(memorydb_state);
    let migrationhub_app = migrationhub::server::create_router(migrationhub_state);
    let mq_app = mq::server::create_router(mq_state);
    let msk_app = msk::server::create_router(msk_state);
    let mwaa_app = mwaa::server::create_router(mwaa_state);
    let neptune_app = neptune::server::create_router(neptune_state);
    let networkfirewall_app = networkfirewall::server::create_router(networkfirewall_state);
    let opensearch_app = opensearch::server::create_router(opensearch_state);
    let organizations_app = organizations::server::create_router(organizations_state);
    let outposts_app = outposts::server::create_router(outposts_state);
    let personalize_app = personalize::server::create_router(personalize_state);
    let pinpoint_app = pinpoint::server::create_router(pinpoint_state);
    let polly_app = polly::server::create_router(polly_state);
    let proton_app = proton::server::create_router(proton_state);
    let qbusiness_app = qbusiness::server::create_router(qbusiness_state);
    let quicksight_app = quicksight::server::create_router(quicksight_state);
    let ram_app = ram::server::create_router(ram_state);
    let rds_app = rds::server::create_router(rds_state);
    let redshift_app = redshift::server::create_router(redshift_state);
    let rekognition_app = rekognition::server::create_router(rekognition_state);
    let route53_app = route53::server::create_router(route53_state);
    let s3_app = s3::server::create_router(s3_state);
    let sagemaker_app = sagemaker::server::create_router(sagemaker_state);
    let secretsmanager_app = secretsmanager::server::create_router(secretsmanager_state);
    let securityhub_app = securityhub::server::create_router(securityhub_state);
    let securitylake_app = securitylake::server::create_router(securitylake_state);
    let servicecatalog_app = servicecatalog::server::create_router(servicecatalog_state);
    let ses_app = ses::server::create_router(ses_state);
    let shield_app = shield::server::create_router(shield_state);
    let sns_app = sns::server::create_router(sns_state);
    let sqs_app = sqs::server::create_router(sqs_state);
    let ssm_app = ssm::server::create_router(ssm_state);
    let stepfunctions_app = stepfunctions::server::create_router(stepfunctions_state);
    let storagegateway_app = storagegateway::server::create_router(storagegateway_state);
    let swf_app = swf::server::create_router(swf_state);
    let textract_app = textract::server::create_router(textract_state);
    let timestream_app = timestream::server::create_router(timestream_state);
    let transcribe_app = transcribe::server::create_router(transcribe_state);
    let transferfamily_app = transferfamily::server::create_router(transferfamily_state);
    let translate_app = translate::server::create_router(translate_state);
    let trustedadvisor_app = trustedadvisor::server::create_router(trustedadvisor_state);
    let verifiedpermissions_app = verifiedpermissions::server::create_router(verifiedpermissions_state);
    let vpclattice_app = vpclattice::server::create_router(vpclattice_state);
    let waf_app = waf::server::create_router(waf_state);
    let workdocs_app = workdocs::server::create_router(workdocs_state);
    let workmail_app = workmail::server::create_router(workmail_state);
    let workspaces_app = workspaces::server::create_router(workspaces_state);
    let xray_app = xray::server::create_router(xray_state);

    macro_rules! spawn_service {
        ($app:expr, $port:expr, $name:expr) => {{
            let port = $port;
            let app = $app;
            tokio::spawn(async move {
                let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("Failed to bind {} service on port {}: {}", $name, port, e);
                        return;
                    }
                };
                println!("{} service listening on port {}", $name, port);
                if let Err(e) = axum::serve(listener, app).await {
                    eprintln!("{} service on port {} exited with error: {}", $name, port, e);
                }
            })
        }};
    }

    let acm_handle = spawn_service!(acm_app, args.acm_port, "ACM");
    let amplify_handle = spawn_service!(amplify_app, args.amplify_port, "Amplify");
    let apigateway_handle = spawn_service!(apigateway_app, args.apigateway_port, "API Gateway");
    let appfabric_handle = spawn_service!(appfabric_app, args.appfabric_port, "AppFabric");
    let appflow_handle = spawn_service!(appflow_app, args.appflow_port, "AppFlow");
    let appmesh_handle = spawn_service!(appmesh_app, args.appmesh_port, "AppMesh");
    let apprunner_handle = spawn_service!(apprunner_app, args.apprunner_port, "AppRunner");
    let appsync_handle = spawn_service!(appsync_app, args.appsync_port, "AppSync");
    let athena_handle = spawn_service!(athena_app, args.athena_port, "Athena");
    let autoscaling_handle = spawn_service!(autoscaling_app, args.autoscaling_port, "AutoScaling");
    let b2bi_handle = spawn_service!(b2bi_app, args.b2bi_port, "B2BI");
    let backup_handle = spawn_service!(backup_app, args.backup_port, "Backup");
    let batch_handle = spawn_service!(batch_app, args.batch_port, "Batch");
    let bedrock_handle = spawn_service!(bedrock_app, args.bedrock_port, "Bedrock");
    let billingconductor_handle = spawn_service!(billingconductor_app, args.billingconductor_port, "BillingConductor");
    let braket_handle = spawn_service!(braket_app, args.braket_port, "Braket");
    let budgets_handle = spawn_service!(budgets_app, args.budgets_port, "Budgets");
    let chime_handle = spawn_service!(chime_app, args.chime_port, "Chime");
    let cleanrooms_handle = spawn_service!(cleanrooms_app, args.cleanrooms_port, "CleanRooms");
    let cloudformation_handle = spawn_service!(cloudformation_app, args.cloudformation_port, "CloudFormation");
    let cloudfront_handle = spawn_service!(cloudfront_app, args.cloudfront_port, "CloudFront");
    let cloudhsm_handle = spawn_service!(cloudhsm_app, args.cloudhsm_port, "CloudHSM");
    let cloudmap_handle = spawn_service!(cloudmap_app, args.cloudmap_port, "CloudMap");
    let cloudsearch_handle = spawn_service!(cloudsearch_app, args.cloudsearch_port, "CloudSearch");
    let cloudtrail_handle = spawn_service!(cloudtrail_app, args.cloudtrail_port, "CloudTrail");
    let cloudwatch_handle = spawn_service!(cloudwatch_app, args.cloudwatch_port, "CloudWatch");
    let cloudwatchlogs_handle = spawn_service!(cloudwatchlogs_app, args.cloudwatchlogs_port, "CloudWatch Logs");
    let codeartifact_handle = spawn_service!(codeartifact_app, args.codeartifact_port, "CodeArtifact");
    let codebuild_handle = spawn_service!(codebuild_app, args.codebuild_port, "CodeBuild");
    let codecatalyst_handle = spawn_service!(codecatalyst_app, args.codecatalyst_port, "CodeCatalyst");
    let codecommit_handle = spawn_service!(codecommit_app, args.codecommit_port, "CodeCommit");
    let codedeploy_handle = spawn_service!(codedeploy_app, args.codedeploy_port, "CodeDeploy");
    let codepipeline_handle = spawn_service!(codepipeline_app, args.codepipeline_port, "CodePipeline");
    let cognito_handle = spawn_service!(cognito_app, args.cognito_port, "Cognito");
    let comprehend_handle = spawn_service!(comprehend_app, args.comprehend_port, "Comprehend");
    let computeoptimizer_handle = spawn_service!(computeoptimizer_app, args.computeoptimizer_port, "ComputeOptimizer");
    let config_handle = spawn_service!(config_app, args.config_port, "Config");
    let connect_handle = spawn_service!(connect_app, args.connect_port, "Connect");
    let controltower_handle = spawn_service!(controltower_app, args.controltower_port, "ControlTower");
    let costexplorer_handle = spawn_service!(costexplorer_app, args.costexplorer_port, "CostExplorer");
    let dataexchange_handle = spawn_service!(dataexchange_app, args.dataexchange_port, "DataExchange");
    let datapipeline_handle = spawn_service!(datapipeline_app, args.datapipeline_port, "DataPipeline");
    let datasync_handle = spawn_service!(datasync_app, args.datasync_port, "DataSync");
    let datazone_handle = spawn_service!(datazone_app, args.datazone_port, "DataZone");
    let detective_handle = spawn_service!(detective_app, args.detective_port, "Detective");
    let devicefarm_handle = spawn_service!(devicefarm_app, args.devicefarm_port, "DeviceFarm");
    let devopsguru_handle = spawn_service!(devopsguru_app, args.devopsguru_port, "DevOpsGuru");
    let directconnect_handle = spawn_service!(directconnect_app, args.directconnect_port, "DirectConnect");
    let directoryservice_handle = spawn_service!(directoryservice_app, args.directoryservice_port, "DirectoryService");
    let dms_handle = spawn_service!(dms_app, args.dms_port, "DMS");
    let documentdb_handle = spawn_service!(documentdb_app, args.documentdb_port, "DocumentDB");
    let drs_handle = spawn_service!(drs_app, args.drs_port, "DRS");
    let dynamodb_handle = spawn_service!(dynamodb_app, args.dynamodb_port, "DynamoDB");
    let ec2_handle = spawn_service!(ec2_app, args.ec2_port, "EC2");
    let ecr_handle = spawn_service!(ecr_app, args.ecr_port, "ECR");
    let ecs_handle = spawn_service!(ecs_app, args.ecs_port, "ECS");
    let efs_handle = spawn_service!(efs_app, args.efs_port, "EFS");
    let eks_handle = spawn_service!(eks_app, args.eks_port, "EKS");
    let elasticache_handle = spawn_service!(elasticache_app, args.elasticache_port, "ElastiCache");
    let elasticbeanstalk_handle = spawn_service!(elasticbeanstalk_app, args.elasticbeanstalk_port, "ElasticBeanstalk");
    let elastictranscoder_handle = spawn_service!(elastictranscoder_app, args.elastictranscoder_port, "ElasticTranscoder");
    let elb_handle = spawn_service!(elb_app, args.elb_port, "ELB");
    let emr_handle = spawn_service!(emr_app, args.emr_port, "EMR");
    let entityresolution_handle = spawn_service!(entityresolution_app, args.entityresolution_port, "EntityResolution");
    let eventbridge_handle = spawn_service!(eventbridge_app, args.eventbridge_port, "EventBridge");
    let finspace_handle = spawn_service!(finspace_app, args.finspace_port, "FinSpace");
    let firehose_handle = spawn_service!(firehose_app, args.firehose_port, "Firehose");
    let firewallmanager_handle = spawn_service!(firewallmanager_app, args.firewallmanager_port, "FirewallManager");
    let fis_handle = spawn_service!(fis_app, args.fis_port, "FIS");
    let forecast_handle = spawn_service!(forecast_app, args.forecast_port, "Forecast");
    let frauddetector_handle = spawn_service!(frauddetector_app, args.frauddetector_port, "FraudDetector");
    let fsx_handle = spawn_service!(fsx_app, args.fsx_port, "FSx");
    let gamelift_handle = spawn_service!(gamelift_app, args.gamelift_port, "GameLift");
    let globalaccelerator_handle = spawn_service!(globalaccelerator_app, args.globalaccelerator_port, "GlobalAccelerator");
    let glue_handle = spawn_service!(glue_app, args.glue_port, "Glue");
    let groundstation_handle = spawn_service!(groundstation_app, args.groundstation_port, "GroundStation");
    let guardduty_handle = spawn_service!(guardduty_app, args.guardduty_port, "GuardDuty");
    let health_handle = spawn_service!(health_app, args.health_port, "Health");
    let healthlake_handle = spawn_service!(healthlake_app, args.healthlake_port, "HealthLake");
    let iam_handle = spawn_service!(iam_app, args.iam_port, "IAM");
    let iamidentitycenter_handle = spawn_service!(iamidentitycenter_app, args.iamidentitycenter_port, "IAMIdentityCenter");
    let imagebuilder_handle = spawn_service!(imagebuilder_app, args.imagebuilder_port, "ImageBuilder");
    let inspector_handle = spawn_service!(inspector_app, args.inspector_port, "Inspector");
    let iotcore_handle = spawn_service!(iotcore_app, args.iotcore_port, "IoTCore");
    let iotevents_handle = spawn_service!(iotevents_app, args.iotevents_port, "IoTEvents");
    let iotfleetwise_handle = spawn_service!(iotfleetwise_app, args.iotfleetwise_port, "IoTFleetWise");
    let iotgreengrass_handle = spawn_service!(iotgreengrass_app, args.iotgreengrass_port, "IoTGreengrass");
    let iotsitewise_handle = spawn_service!(iotsitewise_app, args.iotsitewise_port, "IoTSiteWise");
    let iottwinmaker_handle = spawn_service!(iottwinmaker_app, args.iottwinmaker_port, "IoTTwinMaker");
    let ivs_handle = spawn_service!(ivs_app, args.ivs_port, "IVS");
    let kendra_handle = spawn_service!(kendra_app, args.kendra_port, "Kendra");
    let keyspaces_handle = spawn_service!(keyspaces_app, args.keyspaces_port, "Keyspaces");
    let kinesis_handle = spawn_service!(kinesis_app, args.kinesis_port, "Kinesis");
    let kinesisvideostreams_handle = spawn_service!(kinesisvideostreams_app, args.kinesisvideostreams_port, "KinesisVideoStreams");
    let kms_handle = spawn_service!(kms_app, args.kms_port, "KMS");
    let lakeformation_handle = spawn_service!(lakeformation_app, args.lakeformation_port, "LakeFormation");
    let lambda_handle = spawn_service!(lambda_app, args.lambda_port, "Lambda");
    let lex_handle = spawn_service!(lex_app, args.lex_port, "Lex");
    let licensemanager_handle = spawn_service!(licensemanager_app, args.licensemanager_port, "LicenseManager");
    let lightsail_handle = spawn_service!(lightsail_app, args.lightsail_port, "Lightsail");
    let location_handle = spawn_service!(location_app, args.location_port, "Location");
    let macie_handle = spawn_service!(macie_app, args.macie_port, "Macie");
    let mainframemod_handle = spawn_service!(mainframemod_app, args.mainframemod_port, "MainframeMod");
    let managedblockchain_handle = spawn_service!(managedblockchain_app, args.managedblockchain_port, "ManagedBlockchain");
    let managedflink_handle = spawn_service!(managedflink_app, args.managedflink_port, "ManagedFlink");
    let managedgrafana_handle = spawn_service!(managedgrafana_app, args.managedgrafana_port, "ManagedGrafana");
    let managedprometheus_handle = spawn_service!(managedprometheus_app, args.managedprometheus_port, "ManagedPrometheus");
    let mediaconvert_handle = spawn_service!(mediaconvert_app, args.mediaconvert_port, "MediaConvert");
    let medialive_handle = spawn_service!(medialive_app, args.medialive_port, "MediaLive");
    let mediapackage_handle = spawn_service!(mediapackage_app, args.mediapackage_port, "MediaPackage");
    let mediastore_handle = spawn_service!(mediastore_app, args.mediastore_port, "MediaStore");
    let memorydb_handle = spawn_service!(memorydb_app, args.memorydb_port, "MemoryDB");
    let migrationhub_handle = spawn_service!(migrationhub_app, args.migrationhub_port, "MigrationHub");
    let mq_handle = spawn_service!(mq_app, args.mq_port, "MQ");
    let msk_handle = spawn_service!(msk_app, args.msk_port, "MSK");
    let mwaa_handle = spawn_service!(mwaa_app, args.mwaa_port, "MWAA");
    let neptune_handle = spawn_service!(neptune_app, args.neptune_port, "Neptune");
    let networkfirewall_handle = spawn_service!(networkfirewall_app, args.networkfirewall_port, "NetworkFirewall");
    let opensearch_handle = spawn_service!(opensearch_app, args.opensearch_port, "OpenSearch");
    let organizations_handle = spawn_service!(organizations_app, args.organizations_port, "Organizations");
    let outposts_handle = spawn_service!(outposts_app, args.outposts_port, "Outposts");
    let personalize_handle = spawn_service!(personalize_app, args.personalize_port, "Personalize");
    let pinpoint_handle = spawn_service!(pinpoint_app, args.pinpoint_port, "Pinpoint");
    let polly_handle = spawn_service!(polly_app, args.polly_port, "Polly");
    let proton_handle = spawn_service!(proton_app, args.proton_port, "Proton");
    let qbusiness_handle = spawn_service!(qbusiness_app, args.qbusiness_port, "QBusiness");
    let quicksight_handle = spawn_service!(quicksight_app, args.quicksight_port, "QuickSight");
    let ram_handle = spawn_service!(ram_app, args.ram_port, "RAM");
    let rds_handle = spawn_service!(rds_app, args.rds_port, "RDS");
    let redshift_handle = spawn_service!(redshift_app, args.redshift_port, "Redshift");
    let rekognition_handle = spawn_service!(rekognition_app, args.rekognition_port, "Rekognition");
    let route53_handle = spawn_service!(route53_app, args.route53_port, "Route53");
    let s3_handle = spawn_service!(s3_app, args.s3_port, "S3");
    let sagemaker_handle = spawn_service!(sagemaker_app, args.sagemaker_port, "SageMaker");
    let secretsmanager_handle = spawn_service!(secretsmanager_app, args.secretsmanager_port, "Secrets Manager");
    let securityhub_handle = spawn_service!(securityhub_app, args.securityhub_port, "SecurityHub");
    let securitylake_handle = spawn_service!(securitylake_app, args.securitylake_port, "SecurityLake");
    let servicecatalog_handle = spawn_service!(servicecatalog_app, args.servicecatalog_port, "Service Catalog");
    let ses_handle = spawn_service!(ses_app, args.ses_port, "SES");
    let shield_handle = spawn_service!(shield_app, args.shield_port, "Shield");
    let sns_handle = spawn_service!(sns_app, args.sns_port, "SNS");
    let sqs_handle = spawn_service!(sqs_app, args.sqs_port, "SQS");
    let ssm_handle = spawn_service!(ssm_app, args.ssm_port, "SSM Parameter Store");
    let stepfunctions_handle = spawn_service!(stepfunctions_app, args.stepfunctions_port, "Step Functions");
    let storagegateway_handle = spawn_service!(storagegateway_app, args.storagegateway_port, "StorageGateway");
    let swf_handle = spawn_service!(swf_app, args.swf_port, "SWF");
    let textract_handle = spawn_service!(textract_app, args.textract_port, "Textract");
    let timestream_handle = spawn_service!(timestream_app, args.timestream_port, "Timestream");
    let transcribe_handle = spawn_service!(transcribe_app, args.transcribe_port, "Transcribe");
    let transferfamily_handle = spawn_service!(transferfamily_app, args.transferfamily_port, "TransferFamily");
    let translate_handle = spawn_service!(translate_app, args.translate_port, "Translate");
    let trustedadvisor_handle = spawn_service!(trustedadvisor_app, args.trustedadvisor_port, "TrustedAdvisor");
    let verifiedpermissions_handle = spawn_service!(verifiedpermissions_app, args.verifiedpermissions_port, "VerifiedPermissions");
    let vpclattice_handle = spawn_service!(vpclattice_app, args.vpclattice_port, "VPCLattice");
    let waf_handle = spawn_service!(waf_app, args.waf_port, "WAF");
    let workdocs_handle = spawn_service!(workdocs_app, args.workdocs_port, "WorkDocs");
    let workmail_handle = spawn_service!(workmail_app, args.workmail_port, "WorkMail");
    let workspaces_handle = spawn_service!(workspaces_app, args.workspaces_port, "WorkSpaces");
    let xray_handle = spawn_service!(xray_app, args.xray_port, "XRay");

    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    handles.push(acm_handle);
    handles.push(amplify_handle);
    handles.push(apigateway_handle);
    handles.push(appfabric_handle);
    handles.push(appflow_handle);
    handles.push(appmesh_handle);
    handles.push(apprunner_handle);
    handles.push(appsync_handle);
    handles.push(athena_handle);
    handles.push(autoscaling_handle);
    handles.push(b2bi_handle);
    handles.push(backup_handle);
    handles.push(batch_handle);
    handles.push(bedrock_handle);
    handles.push(billingconductor_handle);
    handles.push(braket_handle);
    handles.push(budgets_handle);
    handles.push(chime_handle);
    handles.push(cleanrooms_handle);
    handles.push(cloudformation_handle);
    handles.push(cloudfront_handle);
    handles.push(cloudhsm_handle);
    handles.push(cloudmap_handle);
    handles.push(cloudsearch_handle);
    handles.push(cloudtrail_handle);
    handles.push(cloudwatch_handle);
    handles.push(cloudwatchlogs_handle);
    handles.push(codeartifact_handle);
    handles.push(codebuild_handle);
    handles.push(codecatalyst_handle);
    handles.push(codecommit_handle);
    handles.push(codedeploy_handle);
    handles.push(codepipeline_handle);
    handles.push(cognito_handle);
    handles.push(comprehend_handle);
    handles.push(computeoptimizer_handle);
    handles.push(config_handle);
    handles.push(connect_handle);
    handles.push(controltower_handle);
    handles.push(costexplorer_handle);
    handles.push(dataexchange_handle);
    handles.push(datapipeline_handle);
    handles.push(datasync_handle);
    handles.push(datazone_handle);
    handles.push(detective_handle);
    handles.push(devicefarm_handle);
    handles.push(devopsguru_handle);
    handles.push(directconnect_handle);
    handles.push(directoryservice_handle);
    handles.push(dms_handle);
    handles.push(documentdb_handle);
    handles.push(drs_handle);
    handles.push(dynamodb_handle);
    handles.push(ec2_handle);
    handles.push(ecr_handle);
    handles.push(ecs_handle);
    handles.push(efs_handle);
    handles.push(eks_handle);
    handles.push(elasticache_handle);
    handles.push(elasticbeanstalk_handle);
    handles.push(elastictranscoder_handle);
    handles.push(elb_handle);
    handles.push(emr_handle);
    handles.push(entityresolution_handle);
    handles.push(eventbridge_handle);
    handles.push(finspace_handle);
    handles.push(firehose_handle);
    handles.push(firewallmanager_handle);
    handles.push(fis_handle);
    handles.push(forecast_handle);
    handles.push(frauddetector_handle);
    handles.push(fsx_handle);
    handles.push(gamelift_handle);
    handles.push(globalaccelerator_handle);
    handles.push(glue_handle);
    handles.push(groundstation_handle);
    handles.push(guardduty_handle);
    handles.push(health_handle);
    handles.push(healthlake_handle);
    handles.push(iam_handle);
    handles.push(iamidentitycenter_handle);
    handles.push(imagebuilder_handle);
    handles.push(inspector_handle);
    handles.push(iotcore_handle);
    handles.push(iotevents_handle);
    handles.push(iotfleetwise_handle);
    handles.push(iotgreengrass_handle);
    handles.push(iotsitewise_handle);
    handles.push(iottwinmaker_handle);
    handles.push(ivs_handle);
    handles.push(kendra_handle);
    handles.push(keyspaces_handle);
    handles.push(kinesis_handle);
    handles.push(kinesisvideostreams_handle);
    handles.push(kms_handle);
    handles.push(lakeformation_handle);
    handles.push(lambda_handle);
    handles.push(lex_handle);
    handles.push(licensemanager_handle);
    handles.push(lightsail_handle);
    handles.push(location_handle);
    handles.push(macie_handle);
    handles.push(mainframemod_handle);
    handles.push(managedblockchain_handle);
    handles.push(managedflink_handle);
    handles.push(managedgrafana_handle);
    handles.push(managedprometheus_handle);
    handles.push(mediaconvert_handle);
    handles.push(medialive_handle);
    handles.push(mediapackage_handle);
    handles.push(mediastore_handle);
    handles.push(memorydb_handle);
    handles.push(migrationhub_handle);
    handles.push(mq_handle);
    handles.push(msk_handle);
    handles.push(mwaa_handle);
    handles.push(neptune_handle);
    handles.push(networkfirewall_handle);
    handles.push(opensearch_handle);
    handles.push(organizations_handle);
    handles.push(outposts_handle);
    handles.push(personalize_handle);
    handles.push(pinpoint_handle);
    handles.push(polly_handle);
    handles.push(proton_handle);
    handles.push(qbusiness_handle);
    handles.push(quicksight_handle);
    handles.push(ram_handle);
    handles.push(rds_handle);
    handles.push(redshift_handle);
    handles.push(rekognition_handle);
    handles.push(route53_handle);
    handles.push(s3_handle);
    handles.push(sagemaker_handle);
    handles.push(secretsmanager_handle);
    handles.push(securityhub_handle);
    handles.push(securitylake_handle);
    handles.push(servicecatalog_handle);
    handles.push(ses_handle);
    handles.push(shield_handle);
    handles.push(sns_handle);
    handles.push(sqs_handle);
    handles.push(ssm_handle);
    handles.push(stepfunctions_handle);
    handles.push(storagegateway_handle);
    handles.push(swf_handle);
    handles.push(textract_handle);
    handles.push(timestream_handle);
    handles.push(transcribe_handle);
    handles.push(transferfamily_handle);
    handles.push(translate_handle);
    handles.push(trustedadvisor_handle);
    handles.push(verifiedpermissions_handle);
    handles.push(vpclattice_handle);
    handles.push(waf_handle);
    handles.push(workdocs_handle);
    handles.push(workmail_handle);
    handles.push(workspaces_handle);
    handles.push(xray_handle);

    // Wait for Ctrl+C to shut down
    match tokio::signal::ctrl_c().await {
        Ok(()) => println!("\nShutting down all services..."),
        Err(e) => eprintln!("Failed to listen for Ctrl+C: {}", e),
    }
}
